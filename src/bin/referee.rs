// Arbitre local : cargo run --release --bin referee -- <test.txt> <bot.exe>
// Rejoue les règles du README et affiche "score max_turn_ms first_turn_ms".
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let text = std::fs::read_to_string(&args[1]).unwrap();
    let mut lines = text.lines();
    let mut nums = |l: &str| l.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let tree = nums(lines.next().unwrap());
    let house = nums(lines.next().unwrap());
    let wh = nums(lines.next().unwrap());
    let fs = nums(lines.next().unwrap());
    let (w, h) = (wh[0] as usize, wh[1] as usize);
    let grid: Vec<Vec<u8>> = (0..h).map(|_| lines.next().unwrap().trim().bytes().collect()).collect();

    // kind: 0 sûr, 1 arbre, 2 maison ; [cut, fire, value]
    let params = [[0, 0, 0], [tree[0], tree[1], tree[2]], [house[0], house[1], house[2]]];
    let kind: Vec<usize> = grid
        .iter()
        .flat_map(|r| r.iter().map(|&c| match c { b'.' => 1, b'X' => 2, _ => 0 }))
        .collect();
    let mut progress: Vec<i32> = kind.iter().map(|&k| if k == 0 { -2 } else { -1 }).collect();
    let fire_dur = |c: usize| params[kind[c]][1];
    progress[fs[1] as usize * w + fs[0] as usize] = 0;

    let mut child = Command::new(&args[2])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    let init: Vec<&str> = text.lines().take(4 + h).collect();
    writeln!(stdin, "{}", init.join("\n")).unwrap();

    let mut cooldown = 0;
    let mut alive = true; // le bot n'a pas fait d'erreur
    let mut max_ms = 0.0f64;
    let mut first_ms = 0.0f64;
    let mut turn = 0;
    loop {
        let burning = (0..w * h).any(|c| progress[c] >= 0 && progress[c] < fire_dur(c));
        if !burning {
            break;
        }
        if alive {
            let mut s = format!("{}\n", cooldown);
            for y in 0..h {
                let row: Vec<String> = (0..w).map(|x| progress[y * w + x].to_string()).collect();
                s += &row.join(" ");
                s.push('\n');
            }
            let t0 = Instant::now();
            stdin.write_all(s.as_bytes()).unwrap();
            stdin.flush().unwrap();
            let mut out = String::new();
            stdout.read_line(&mut out).unwrap();
            let ms = t0.elapsed().as_secs_f64() * 1000.0;
            if turn == 0 { first_ms = ms } else { max_ms = max_ms.max(ms) }
            let out = out.trim();
            if out != "WAIT" {
                let v: Vec<usize> = out.split_whitespace().map(|v| v.parse().unwrap()).collect();
                let c = v[1] * w + v[0];
                if cooldown > 0 || progress[c] != -1 {
                    eprintln!("INVALID action '{}' turn {} (cooldown {}, progress {})", out, turn, cooldown, progress[c]);
                    alive = false;
                } else {
                    progress[c] = -2;
                    cooldown = params[kind[c]][0];
                }
            }
        }
        // propagation : les cellules en feu progressent, puis les brûlées allument leurs voisines
        let mut stack = vec![];
        for c in 0..w * h {
            if progress[c] >= 0 && progress[c] < fire_dur(c) {
                progress[c] += 1;
                if progress[c] == fire_dur(c) {
                    stack.push(c);
                }
            }
        }
        while let Some(c) = stack.pop() {
            for n in [c - 1, c + 1, c - w, c + w] {
                if progress[n] == -1 {
                    progress[n] = 0;
                    if fire_dur(n) == 0 {
                        stack.push(n);
                    }
                }
            }
        }
        cooldown = (cooldown - 1).max(0);
        turn += 1;
    }
    drop(stdin);
    let _ = child.kill();
    // Les cellules rasées ont progress -2 comme les sûres : on compare au type d'origine
    let score: i32 = (0..w * h).filter(|&c| kind[c] != 0 && progress[c] == -1).map(|c| params[kind[c]][2]).sum();
    println!("{} {:.1} {:.0}", score, max_ms, first_ms);
}
