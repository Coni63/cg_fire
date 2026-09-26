use std::cmp::Reverse;
use std::collections::VecDeque;
use std::io;
use std::time::{Duration, Instant};

const NONE: i32 = i32::MAX;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn read_ints() -> Vec<i32> {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line.split_whitespace().map(|v| parse_input!(v, i32)).collect()
}

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
    fn f64(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// Données fixes de la partie. Les cellules sont indexées par y * w + x.
struct Game {
    w: usize,
    kind: Vec<usize>, // 0 = sûre, 1 = arbre, 2 = maison
    cut_dur: [i32; 3],
    fire_dur: [i32; 3],
    value: [i32; 3],
    nb4: Vec<Vec<usize>>, // voisins non sûrs (4 directions)
    nb8: Vec<Vec<usize>>, // voisins non sûrs (8 directions), pour les mouvements
}

impl Game {
    fn value(&self, c: usize) -> i32 {
        self.value[self.kind[c]]
    }
    fn fire(&self, c: usize) -> i32 {
        self.fire_dur[self.kind[c]]
    }
    /// Un tour minimum par coupe, même si cutDuration vaut 0.
    fn step(&self, c: usize) -> i32 {
        self.cut_dur[self.kind[c]].max(1)
    }
}

/// Photo de la partie au tour courant (temps relatifs : 0 = maintenant).
struct State {
    ignite: Vec<i32>, // instant d'allumage (<= 0) des cellules déjà en feu / brûlées
    cut: Vec<bool>,   // cellules déjà rasées
    cooldown: i32,
}

impl State {
    fn from_input(g: &Game, progress: &[i32], cooldown: i32) -> State {
        let n = progress.len();
        let mut st = State { ignite: vec![NONE; n], cut: vec![false; n], cooldown };
        for c in 0..n {
            if g.kind[c] == 0 {
                continue;
            }
            match progress[c] {
                -2 => st.cut[c] = true,
                -1 => {}
                p => st.ignite[c] = -p,
            }
        }
        st
    }
}

/// File à buckets circulaire : les durées de feu sont <= 10, donc tous les événements
/// en attente tiennent dans une fenêtre de 32 tours.
struct Buckets {
    b: Vec<Vec<u32>>,
    cur: i32,
    count: usize,
}

impl Buckets {
    fn clear(&mut self, cur: i32) {
        self.b.iter_mut().for_each(|v| v.clear());
        self.cur = cur;
        self.count = 0;
    }
    fn push(&mut self, t: i32, c: usize) {
        self.b[(t as u32 & 31) as usize].push(c as u32);
        self.count += 1;
    }
    /// (instant, cellule) du prochain événement, sans le retirer
    fn peek(&mut self) -> Option<(i32, usize)> {
        if self.count == 0 {
            return None;
        }
        loop {
            if let Some(&c) = self.b[(self.cur as u32 & 31) as usize].last() {
                return Some((self.cur, c as usize));
            }
            self.cur += 1;
        }
    }
    fn pop(&mut self) {
        self.b[(self.cur as u32 & 31) as usize].pop();
        self.count -= 1;
    }
}

/// Simulation exacte d'un plan : Dijkstra de propagation entrelacé avec les coupes.
/// Une coupe démarrée à t bloque le feu qui arriverait à d > t. Une coupe sur une cellule
/// déjà en feu est ignorée (sans coût), comme le fera l'exécution en jeu.
struct Sim {
    ignite: Vec<i32>,
    cut: Vec<bool>,
    threat: Vec<i32>, // pour une cellule rasée : instant où le feu l'aurait atteinte
    queue: Buckets,
    executed: Vec<bool>,
    base: i32,
}

impl Sim {
    fn new(g: &Game, st: &State) -> Sim {
        let n = g.kind.len();
        let base = (0..n).filter(|&c| !st.cut[c]).map(|c| g.value(c)).sum();
        Sim {
            ignite: vec![NONE; n],
            cut: vec![false; n],
            threat: vec![NONE; n],
            queue: Buckets { b: vec![vec![]; 32], cur: 0, count: 0 },
            executed: vec![],
            base,
        }
    }

    fn run(&mut self, g: &Game, st: &State, plan: &[u32]) -> i32 {
        self.ignite.fill(NONE);
        self.threat.fill(NONE);
        self.cut.copy_from_slice(&st.cut);
        self.executed.clear();
        let first = st.ignite.iter().copied().min().unwrap_or(0);
        self.queue.clear(first);
        for (c, &t) in st.ignite.iter().enumerate() {
            if t != NONE {
                self.queue.push(t, c);
            }
        }

        let mut lost = 0;
        let mut t = st.cooldown;
        let mut i = 0;
        loop {
            while let Some((d, c)) = self.queue.peek() {
                if self.ignite[c] == NONE && !self.cut[c] {
                    break;
                }
                if self.cut[c] {
                    self.threat[c] = self.threat[c].min(d);
                }
                self.queue.pop();
            }
            let Some((d, c)) = self.queue.peek() else { break };

            if i < plan.len() && t < d {
                let p = plan[i] as usize;
                i += 1;
                let ok = !self.cut[p] && self.ignite[p] == NONE;
                self.executed.push(ok);
                if ok {
                    self.cut[p] = true;
                    lost += g.value(p);
                    t += g.step(p);
                }
                continue;
            }

            self.queue.pop();
            self.ignite[c] = d;
            lost += g.value(c);
            let nd = d + g.fire(c);
            for &n in &g.nb4[c] {
                if self.ignite[n] == NONE {
                    if self.cut[n] {
                        self.threat[n] = self.threat[n].min(nd);
                    } else {
                        self.queue.push(nd, n);
                    }
                }
            }
        }
        self.base - lost
    }

    /// Garde uniquement les coupes réellement exécutées (les autres n'ont aucun effet).
    fn clean(&self, plan: &mut Vec<u32>) {
        let mut k = 0;
        plan.retain(|_| {
            let keep = k < self.executed.len() && self.executed[k];
            k += 1;
            keep
        });
    }
}

/// Réordonne le plan par échéance réelle (instant où le feu aurait touché chaque coupe),
/// puis resimule. Répété tant que ça améliore.
fn edd_repair(g: &Game, st: &State, sim: &mut Sim, plan: &mut Vec<u32>, mut score: i32) -> i32 {
    for _ in 0..4 {
        let mut cand = plan.clone();
        cand.sort_by_key(|&c| sim.threat[c as usize]);
        let s = sim.run(g, st, &cand);
        if s <= score {
            sim.run(g, st, plan);
            break;
        }
        score = s;
        sim.clean(&mut cand);
        *plan = cand;
    }
    score
}

/// Graines : couronnes à distance r d'un centre (ton solveur d'origine), triées par
/// instant d'arrivée du feu sans coupe, puis évaluées avec la vraie simulation.
fn ring_seeds(g: &Game, st: &State, sim: &mut Sim, rng: &mut Rng, until: Instant) -> Vec<(i32, Vec<u32>)> {
    let n = g.kind.len();
    let free: Vec<usize> = (0..n).filter(|&c| g.kind[c] != 0).collect();
    sim.run(g, st, &[]);
    let arrival = sim.ignite.clone();

    let mut seeds: Vec<(i32, Vec<u32>)> = vec![(sim.run(g, st, &[]), vec![])];
    let mut dist = vec![u32::MAX; n];
    let mut queue = VecDeque::new();
    let mut tried = vec![false; n];
    let mut rings: Vec<Vec<u32>> = vec![];

    while Instant::now() < until && tried.iter().filter(|&&b| b).count() < free.len() {
        let core = free[rng.below(free.len())];
        if tried[core] {
            continue;
        }
        tried[core] = true;

        dist.fill(u32::MAX);
        dist[core] = 0;
        queue.push_back(core);
        let mut max_d = 0;
        while let Some(c) = queue.pop_front() {
            max_d = max_d.max(dist[c]);
            for &nb in &g.nb4[c] {
                if dist[nb] == u32::MAX {
                    dist[nb] = dist[c] + 1;
                    queue.push_back(nb);
                }
            }
        }
        rings.iter_mut().for_each(|r| r.clear());
        rings.resize(max_d as usize + 1, vec![]);
        for &c in &free {
            if dist[c] != u32::MAX && arrival[c] != NONE && !st.cut[c] {
                rings[dist[c] as usize].push(c as u32);
            }
        }
        for ring in rings.iter_mut().skip(1) {
            if ring.is_empty() {
                continue;
            }
            ring.sort_by_key(|&c| arrival[c as usize]);
            let mut plan = ring.clone();
            let s = sim.run(g, st, &plan);
            let s = edd_repair(g, st, sim, &mut plan, s);
            if s > seeds[seeds.len() - 1].0 || seeds.len() < 8 {
                sim.run(g, st, &plan);
                sim.clean(&mut plan);
                if !seeds.iter().any(|(_, p)| *p == plan) {
                    seeds.push((s, plan));
                    seeds.sort_by_key(|(s, _)| Reverse(*s));
                    seeds.truncate(8);
                }
            }
        }
    }
    seeds
}

fn random_neighbor(g: &Game, st: &State, plan: &[u32], rng: &mut Rng, c: usize) -> Option<u32> {
    let nbs = &g.nb8[c];
    if nbs.is_empty() {
        return None;
    }
    let n = nbs[rng.below(nbs.len())];
    if st.cut[n] || plan.contains(&(n as u32)) {
        return None;
    }
    Some(n as u32)
}

/// Cellules non sûres à distance exactement r de `center` (BFS 4 directions).
fn ring_around(g: &Game, center: usize, r: usize) -> Vec<usize> {
    let mut layer = vec![center];
    let mut seen = vec![center];
    for _ in 0..r {
        let mut next = vec![];
        for &c in &layer {
            for &n in &g.nb4[c] {
                if !seen.contains(&n) {
                    seen.push(n);
                    next.push(n);
                }
            }
        }
        layer = next;
    }
    layer
}

/// Recuit simulé sur la séquence de coupes.
fn anneal(g: &Game, st: &State, start: Vec<u32>, until: Instant, t0: f64, t1: f64, rng: &mut Rng) -> (i32, Vec<u32>) {
    let mut sim = Sim::new(g, st);
    let mut cur = start;
    let mut cur_score = sim.run(g, st, &cur);
    sim.clean(&mut cur);
    let mut cur_ignite = sim.ignite.clone();
    let mut cur_threat = sim.threat.clone();
    let mut best = (cur_score, cur.clone());

    let burning: Vec<usize> = (0..g.kind.len()).filter(|&c| g.kind[c] != 0).collect();
    let houses: Vec<usize> = (0..g.kind.len()).filter(|&c| g.kind[c] == 2).collect();
    let begin = Instant::now();
    let total = until.saturating_duration_since(begin).as_secs_f64().max(1e-6);
    let mut temp = t0;
    let mut iter = 0u64;

    loop {
        if iter % 64 == 0 {
            let el = begin.elapsed().as_secs_f64() / total;
            if el >= 1.0 {
                break;
            }
            temp = t0 * (t1 / t0).powf(el);
        }
        iter += 1;

        let mut cand = cur.clone();
        let len = cand.len();
        let r = rng.below(100);
        if r < 8 {
            // entourer une zone : couronne de rayon 1 à 3 autour d'une cellule (maison de préférence)
            let pool = if !houses.is_empty() && rng.below(2) == 0 { &houses } else { &burning };
            let center = pool[rng.below(pool.len())];
            if cur_ignite[center] == NONE || cur_ignite[center] < 0 {
                continue;
            }
            let ring = ring_around(g, center, 1 + rng.below(3));
            for c in ring {
                let key = cur_ignite[c];
                if key == NONE || key < 0 || cand.contains(&(c as u32)) {
                    continue;
                }
                let pos = cand.iter().position(|&p| cur_threat[p as usize] > key).unwrap_or(cand.len());
                cand.insert(pos, c as u32);
            }
        } else if r < 12 {
            // retirer tout un groupe de coupes proches
            if len == 0 {
                continue;
            }
            let p = cand[rng.below(len)] as usize;
            let (px, py) = ((p % g.w) as i64, (p / g.w) as i64);
            cand.retain(|&c| {
                let (x, y) = ((c as usize % g.w) as i64, (c as usize / g.w) as i64);
                (x - px).abs().max((y - py).abs()) > 2
            });
        } else if r < 35 || len == 0 {
            // insertion : voisin d'une coupe existante, ou cellule quelconque qui brûle
            let c = if len > 0 && rng.below(3) > 0 {
                let base = cand[rng.below(len)] as usize;
                random_neighbor(g, st, &cand, rng, base)
            } else {
                let c = burning[rng.below(burning.len())];
                (cur_ignite[c] != NONE && cur_ignite[c] >= 0 && !cand.contains(&(c as u32))).then_some(c as u32)
            };
            let Some(c) = c else { continue };
            let key = cur_ignite[c as usize];
            let pos = if rng.below(4) == 0 {
                rng.below(len + 1)
            } else {
                cand.iter().position(|&p| cur_threat[p as usize] > key).unwrap_or(len)
            };
            cand.insert(pos, c);
        } else if r < 45 {
            cand.remove(rng.below(len));
        } else if r < 70 {
            // déplacement latéral d'une coupe
            let i = rng.below(len);
            let Some(c) = random_neighbor(g, st, &cand, rng, cand[i] as usize) else { continue };
            cand[i] = c;
        } else if r < 85 {
            if len < 2 {
                continue;
            }
            let i = rng.below(len - 1);
            cand.swap(i, i + 1);
        } else if r < 95 {
            // changer une coupe de place dans l'ordre
            let i = rng.below(len);
            let c = cand.remove(i);
            let j = if rng.below(2) == 0 {
                rng.below(len)
            } else {
                (i as i64 + rng.below(9) as i64 - 4).clamp(0, len as i64 - 1) as usize
            };
            cand.insert(j, c);
        } else {
            // retirer une coupe et en ajouter une à côté d'une autre
            cand.remove(rng.below(len));
            if cand.is_empty() {
                continue;
            }
            let i = rng.below(cand.len());
            let Some(c) = random_neighbor(g, st, &cand, rng, cand[i] as usize) else { continue };
            cand.insert(i + rng.below(2), c);
        }

        let s = sim.run(g, st, &cand);
        if s >= cur_score || rng.f64() < ((s - cur_score) as f64 / temp).exp() {
            sim.clean(&mut cand);
            cur = cand;
            cur_score = s;
            cur_ignite.copy_from_slice(&sim.ignite);
            cur_threat.copy_from_slice(&sim.threat);
            if s > best.0 {
                best = (s, cur.clone());
            }
        }
    }
    let mut plan = best.1;
    let score = sim.run(g, st, &plan);
    let score = edd_repair(g, st, &mut sim, &mut plan, score);
    (score, plan)
}

/// Solution représentée par l'ensemble S des cellules à sauver (idée du forum).
/// Le périmètre P = voisins de S hors S doit être rasé. Tant qu'aucune cellule de P n'a
/// brûlé, le feu ne circule que dehors : l'échéance de chaque cellule de P se calcule donc
/// avec toutes les coupes en place, et l'ordre par échéance croissante est optimal.
#[derive(Clone)]
struct Region {
    in_s: Vec<bool>,
    list: Vec<usize>,
}

impl Region {
    fn new(n: usize) -> Region {
        Region { in_s: vec![false; n], list: vec![] }
    }
    fn add(&mut self, c: usize) {
        if !self.in_s[c] {
            self.in_s[c] = true;
            self.list.push(c);
        }
    }
    fn remove(&mut self, c: usize) {
        if self.in_s[c] {
            self.in_s[c] = false;
            let i = self.list.iter().position(|&x| x == c).unwrap();
            self.list.swap_remove(i);
        }
    }
}

struct RegionEval {
    mark: Vec<u32>,
    stamp: u32,
    ignite: Vec<i32>,
    threat: Vec<i32>,
    queue: Buckets,
    perim: Vec<usize>,
    cuts: Vec<usize>,
    base: i32,
    first: i32,
}

impl RegionEval {
    fn new(g: &Game, st: &State) -> RegionEval {
        let n = g.kind.len();
        RegionEval {
            mark: vec![0; n],
            stamp: 0,
            ignite: vec![NONE; n],
            threat: vec![NONE; n],
            queue: Buckets { b: vec![vec![]; 32], cur: 0, count: 0 },
            perim: vec![],
            cuts: vec![],
            base: (0..n).filter(|&c| !st.cut[c]).map(|c| g.value(c)).sum(),
            first: st.ignite.iter().copied().min().unwrap_or(0),
        }
    }

    /// Score de la région, ou None si aucun ordre de coupe ne la protège.
    /// Remplit `perim` (périmètre) et `cuts` (coupes utiles, dans l'ordre).
    fn eval(&mut self, g: &Game, st: &State, r: &Region) -> Option<i32> {
        self.stamp += 1;
        let stamp = self.stamp;
        self.perim.clear();
        for &c in &r.list {
            if st.ignite[c] != NONE {
                return None;
            }
            for &n in &g.nb4[c] {
                if !r.in_s[n] && !st.cut[n] && self.mark[n] != stamp {
                    if st.ignite[n] != NONE {
                        return None;
                    }
                    self.mark[n] = stamp;
                    self.threat[n] = NONE;
                    self.perim.push(n);
                }
            }
        }

        self.ignite.fill(NONE);
        self.queue.clear(self.first);
        for (c, &t) in st.ignite.iter().enumerate() {
            if t != NONE {
                self.queue.push(t, c);
            }
        }
        let mut lost = 0;
        while let Some((d, c)) = self.queue.peek() {
            self.queue.pop();
            if self.ignite[c] != NONE {
                continue;
            }
            self.ignite[c] = d;
            lost += g.value(c);
            let nd = d + g.fire(c);
            for &n in &g.nb4[c] {
                if self.ignite[n] != NONE || st.cut[n] {
                    continue;
                }
                if self.mark[n] == stamp {
                    self.threat[n] = self.threat[n].min(nd);
                } else {
                    self.queue.push(nd, n);
                }
            }
        }

        // Les cellules du périmètre jamais menacées n'ont pas besoin d'être rasées
        self.cuts.clear();
        self.cuts.extend(self.perim.iter().copied().filter(|&p| self.threat[p] != NONE));
        let threat = &self.threat;
        self.cuts.sort_unstable_by_key(|&p| threat[p]);
        let mut t = st.cooldown;
        for &p in &self.cuts {
            if t >= self.threat[p] {
                return None;
            }
            t += g.step(p);
            lost += g.value(p);
        }
        Some(self.base - lost)
    }
}

/// Recuit sur les régions : on agrandit / réduit S en ne gardant que des régions
/// protégeables. Redémarre régulièrement, depuis la meilleure région ou une cellule au hasard.
fn region_search(g: &Game, st: &State, until: Instant, burst: usize, t0: f64, t1: f64, rng: &mut Rng) -> (i32, Vec<u32>) {
    let n = g.kind.len();
    let mut ev = RegionEval::new(g, st);
    let cells: Vec<usize> = (0..n).filter(|&c| g.kind[c] != 0 && !st.cut[c] && st.ignite[c] == NONE).collect();
    let houses: Vec<usize> = cells.iter().copied().filter(|&c| g.kind[c] == 2).collect();
    let empty = Region::new(n);
    let mut best_score = ev.eval(g, st, &empty).unwrap_or(0);
    let mut best = empty.clone();
    if cells.is_empty() {
        return (best_score, vec![]);
    }

    let (mut restarts, mut iters) = (0, 0u64);
    loop {
        // Point de départ : la meilleure région, ou une nouvelle cellule (maison de préférence)
        let mut cur = if restarts % 2 == 1 && !best.list.is_empty() {
            best.clone()
        } else {
            let pool = if !houses.is_empty() && rng.below(2) == 0 { &houses } else { &cells };
            let mut r = Region::new(n);
            r.add(pool[rng.below(pool.len())]);
            r
        };
        if std::env::var("DBG").is_ok() {
            eprintln!("restart {} best {}", restarts, best_score);
        }
        restarts += 1;
        if Instant::now() >= until {
            break;
        }
        let Some(mut cur_score) = ev.eval(g, st, &cur) else { continue };
        let mut cur_perim = ev.perim.clone();
        let mut local = (cur_score, cur.clone());
        let mut timeout = false;

        for k in 0..burst {
            if k % 64 == 0 && Instant::now() >= until {
                timeout = true;
                break;
            }
            iters += 1;
            let temp = t0 * (t1 / t0).powf(k as f64 / burst as f64);
            let r = rng.below(100);
            let (added, c) = if r < 60 && !cur_perim.is_empty() {
                (true, cur_perim[rng.below(cur_perim.len())])
            } else if r < 85 && !cur.list.is_empty() {
                (false, cur.list[rng.below(cur.list.len())])
            } else {
                let c = cells[rng.below(cells.len())];
                if cur.in_s[c] {
                    continue;
                }
                (true, c)
            };
            if added {
                cur.add(c)
            } else {
                cur.remove(c)
            }

            match ev.eval(g, st, &cur) {
                Some(s) if s >= cur_score || rng.f64() < ((s - cur_score) as f64 / temp).exp() => {
                    cur_score = s;
                    cur_perim.clone_from(&ev.perim);
                    if s > local.0 {
                        local = (s, cur.clone());
                    }
                }
                _ => {
                    if added {
                        cur.remove(c)
                    } else {
                        cur.add(c)
                    }
                }
            }
        }

        // Fusion avec la meilleure région : permet de sauver plusieurs zones disjointes
        if local.0 > best_score {
            best_score = local.0;
            std::mem::swap(&mut best, &mut local.1);
        }
        let mut merged = best.clone();
        for &c in &local.1.list {
            merged.add(c);
        }
        if let Some(s) = ev.eval(g, st, &merged) {
            if s > best_score {
                best_score = s;
                best = merged;
            }
        }
        if timeout {
            break;
        }
    }
    let s = ev.eval(g, st, &best).unwrap();
    if std::env::var("DBG").is_ok() {
        for y in 0..n / g.w {
            let row: String = (0..g.w)
                .map(|x| {
                    let c = y * g.w + x;
                    if g.kind[c] == 0 {
                        '#'
                    } else if ev.cuts.contains(&c) {
                        'C'
                    } else if ev.ignite[c] != NONE {
                        if g.kind[c] == 2 { 'F' } else { ',' }
                    } else if g.kind[c] == 2 {
                        'X'
                    } else {
                        '.'
                    }
                })
                .collect();
            eprintln!("{}", row);
        }
    }
    eprintln!("region: {} ({} restarts, {} iters, |S| = {})", s, restarts, iters, best.list.len());
    (s, ev.cuts.iter().map(|&c| c as u32).collect())
}

fn solve_first(g: &Game, st: &State, rng: &mut Rng, until: Instant) -> Vec<u32> {
    let now = Instant::now();
    let total = until.saturating_duration_since(now);
    let mut sim = Sim::new(g, st);
    // Température à l'échelle de la valeur moyenne d'une cellule
    let free = g.kind.iter().filter(|&&k| k != 0).count().max(1);
    let avg = sim.base as f64 / free as f64;
    let env = |k: &str, d: f64| std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d);
    let (t0, t1) = ((avg * env("T0", 3.0)).max(1.0), (avg * env("T1", 0.2)).max(0.05));

    // Portefeuille : chaque carte préfère des réglages différents, on garde le meilleur.
    // (taille des rafales, T0, T1) pour le recuit sur les régions, puis couronnes + séquence.
    let configs = [(5000, 3.0, 0.2), (5000, 10.0, 0.2), (1000, 3.0, 0.2), (5000, 1.0, 0.05)];
    let slot = total.mul_f64(0.8 / (configs.len() + 1) as f64);
    let mut best = (sim.run(g, st, &[]), vec![]);
    for (k, &(burst, a, b)) in configs.iter().enumerate() {
        let (_, plan) = region_search(g, st, now + slot * (k as u32 + 1), burst, avg * a, avg * b, rng);
        let s = sim.run(g, st, &plan);
        if s > best.0 {
            best = (s, plan);
        }
    }
    let seeds = ring_seeds(g, st, &mut sim, rng, Instant::now() + slot / 3);
    let (s, plan) = anneal(g, st, seeds[0].1.clone(), now + slot * (configs.len() as u32 + 1), t0, t1, rng);
    eprintln!("rings + sequence: {}", s);
    if s > best.0 {
        best = (s, plan);
    }

    // Affinage final de la séquence de coupes
    let (s, plan) = anneal(g, st, best.1.clone(), until, t0 * 0.3, t1, rng);
    eprintln!("sequence: {}", s);
    if s > best.0 {
        best = (s, plan);
    }
    eprintln!("best {} ({} cuts)", best.0, best.1.len());
    best.1
}

fn main() {
    let start = Instant::now();
    let tree = read_ints();
    let house = read_ints();
    let wh = read_ints();
    let (w, h) = (wh[0] as usize, wh[1] as usize);
    let _fire_start = read_ints();

    let mut kind = vec![0; w * h];
    for y in 0..h {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        for (x, ch) in input_line.trim().bytes().enumerate().take(w) {
            kind[y * w + x] = match ch {
                b'.' => 1,
                b'X' => 2,
                _ => 0,
            };
        }
    }

    let mut nb4 = vec![vec![]; w * h];
    let mut nb8 = vec![vec![]; w * h];
    for y in 0..h as i32 {
        for x in 0..w as i32 {
            let c = y as usize * w + x as usize;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let (nx, ny) = (x + dx, y + dy);
                    if (dx, dy) == (0, 0) || nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 {
                        continue;
                    }
                    let n = ny as usize * w + nx as usize;
                    if kind[n] == 0 {
                        continue;
                    }
                    nb8[c].push(n);
                    if dx == 0 || dy == 0 {
                        nb4[c].push(n);
                    }
                }
            }
        }
    }

    let g = Game {
        w,
        kind,
        cut_dur: [0, tree[0], house[0]],
        fire_dur: [0, tree[1], house[1]],
        value: [0, tree[2], house[2]],
        nb4,
        nb8,
    };
    let mut rng = Rng(0x9E3779B97F4A7C15);
    let first_budget: u64 = std::env::var("FIRE_MS").ok().and_then(|v| v.parse().ok()).unwrap_or(4600);
    let mut plan: Vec<u32> = vec![];
    let mut first = true;

    loop {
        let mut input_line = String::new();
        if io::stdin().read_line(&mut input_line).unwrap() == 0 {
            return;
        }
        let turn_start = Instant::now();
        let cooldown = parse_input!(input_line, i32);
        let mut progress = Vec::with_capacity(w * h);
        for _ in 0..h {
            progress.extend(read_ints());
        }
        let st = State::from_input(&g, &progress, cooldown);

        if first {
            plan = solve_first(&g, &st, &mut rng, start + Duration::from_millis(first_budget));
            first = false;
        } else {
            // On repart de l'état réel : corrige toute divergence et affine le reste du plan
            let mut sim = Sim::new(&g, &st);
            let cur = sim.run(&g, &st, &plan);
            let (s, p) = anneal(&g, &st, plan.clone(), turn_start + Duration::from_millis(45), 2.0, 0.1, &mut rng);
            if s > cur {
                plan = p;
            }
        }

        let fire_alive = (0..w * h).any(|c| progress[c] >= 0 && progress[c] < g.fire(c));
        let mut action = None;
        if cooldown == 0 && fire_alive {
            while !plan.is_empty() {
                let c = plan.remove(0) as usize;
                if progress[c] == -1 {
                    action = Some(c);
                    break;
                }
            }
        }
        match action {
            Some(c) => println!("{} {}", c % w, c / w),
            None => println!("WAIT"),
        }
    }
}
