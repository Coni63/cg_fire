use rand::{self, Rng};
use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

use crate::board::{Board, Cell};

fn bfs(board: &Board, core_idx: usize) -> HashMap<usize, usize> {
    let dirs = [-1, 1, -50, 50];

    let mut ans = HashMap::new();
    ans.insert(core_idx, 0);

    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(core_idx);

    while let Some(idx) = queue.pop_front() {
        for d_idx in dirs.iter() {
            let n_idx = (idx as i32 + d_idx) as usize;
            if ans.contains_key(&n_idx) {
                continue;
            }

            if board.get_cell(n_idx) == &Cell::Empty {
                continue;
            }

            ans.insert(n_idx, ans.get(&idx).unwrap() + 1);
            queue.push_back(n_idx);
        }
    }

    ans
}

fn get_boundary(board: &Board, core_bfs: &HashMap<usize, usize>, radius: usize) -> Vec<usize> {
    let mut boundary: Vec<usize> = core_bfs
        .iter()
        .filter(|(_, &v)| v == radius)
        .map(|(&k, _)| k)
        .collect();

    boundary.sort_by_cached_key(|idx| board.get_reached_duration(*idx));
    boundary
}

fn is_valid(board: &Board, boundary: &[usize]) -> bool {
    let mut t: i32 = 0;

    for &idx in boundary.iter() {
        if board.get_reached_duration(idx) as i32 <= t {
            return false;
        }

        t += board.get_cut_duration(idx);
    }
    true
}

fn get_score(board: &Board, core_bfs: &HashMap<usize, usize>, radius: usize) -> i32 {
    let mut score = 0;
    let d = core_bfs.get(&board.get_fire_start()).unwrap_or(&0);
    if *d < radius {
        // outside square
        for (&idx, &dist) in core_bfs.iter() {
            if dist > radius {
                score += board.get_value(idx);
            }
        }
    } else {
        // inside_square
        for (&idx, &dist) in core_bfs.iter() {
            if dist < radius {
                score += board.get_value(idx);
            }
        }
    }

    score
}

pub fn solve(board: &mut Board) -> Vec<usize> {
    let timer = Instant::now();
    let mut rng = rand::thread_rng();

    let mut best_boundary = vec![];
    let mut best_boundary_score = -1;
    let mut simulations = 0;
    let max_radius = std::cmp::max(board.get_width(), board.get_height());
    let mut cache: Vec<Option<HashMap<usize, usize>>> = (0..2500).map(|_| None).collect();

    while timer.elapsed().as_millis() < 4950 {
        let core_x = rng.gen_range(1..board.get_width() - 1);
        let core_y = rng.gen_range(1..board.get_height() - 1);
        let core_idx = core_y * 50 + core_x;
        let radius = rng.gen_range(1..max_radius);

        let core_bfs = match cache.get(core_idx).unwrap() {
            Some(x) => x,
            None => {
                let res = bfs(board, core_idx);
                cache[core_idx] = Some(res);
                cache.get(core_idx).unwrap().as_ref().unwrap()
            }
        };
        let boundary = get_boundary(board, core_bfs, radius);

        if is_valid(board, &boundary) {
            let score = get_score(board, core_bfs, radius);
            if score > best_boundary_score {
                best_boundary_score = score;
                best_boundary = boundary;
            }
            simulations += 1;
            // break;
        }
    }

    eprintln!("{}", cache.len());

    eprintln!("{} simulations in {:?}", simulations, timer.elapsed());
    best_boundary
}
