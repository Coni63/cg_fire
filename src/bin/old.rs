use rand::Rng;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};
use std::time::Instant;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

type Coord = (usize, usize);
type Point = (usize, usize, usize); // (x, y, radius)

fn bfs(grid: &[Vec<char>], start: Coord, tree_time: i32, house_time: i32) -> HashMap<Coord, i32> {
    let mut dist = HashMap::new();
    dist.insert(start, 0);
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some((x, y)) = queue.pop_back() {
        let burn_time = match grid[y][x] {
            '.' => tree_time,
            'X' => house_time,
            _ => panic!("Invalid cell"),
        };

        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 || ny as usize >= grid.len() || nx as usize >= grid[0].len() {
                continue;
            }
            let nc = (nx as usize, ny as usize);
            if dist.contains_key(&nc) {
                continue;
            }
            if grid[nc.1][nc.0] == '#' {
                continue;
            }
            dist.insert(nc, dist[&(x, y)] + burn_time);
            queue.push_front(nc);
        }
    }
    dist
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let tree_treatment_duration = parse_input!(inputs[0], i32); // cooldown for cutting a "tree" cell
    let tree_fire_duration = parse_input!(inputs[1], i32); // number of turns for the fire to propagate on adjacent cells from a "tree" cell
    let tree_value = parse_input!(inputs[2], i32); // value lost if a "tree" cell is burnt or cut

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let house_treatment_duration = parse_input!(inputs[0], i32); // cooldown for cutting a "house" cell
    let house_fire_duration = parse_input!(inputs[1], i32); // number of turns for the fire to propagate on adjacent cells from a "house" cell
    let house_value = parse_input!(inputs[2], i32); // value lost if a "house" cell is burnt or cut

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let width = parse_input!(inputs[0], usize); // number of columns in the grid
    let height = parse_input!(inputs[1], usize); // number of rows in the grid
    let max_radius = width.max(height);

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let fire_start_x = parse_input!(inputs[0], usize); // column where the fire starts
    let fire_start_y = parse_input!(inputs[1], usize); // row where the fire starts

    let mut grid = Vec::new();
    for i in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let grid_line = input_line.trim().to_string();
        grid.push(grid_line.chars().collect::<Vec<_>>());
    }

    let mut tried: HashSet<usize> = HashSet::new();
    let dists = bfs(
        &grid,
        (fire_start_x, fire_start_y),
        tree_fire_duration,
        house_fire_duration,
    );

    let start = Instant::now();
    let mut best_boundary = Vec::new();
    let mut best_score = -1;
    let mut simulations = 0;
    let mut skipped_simulations = 0;
    let mut rng = rand::thread_rng();

    while start.elapsed().as_secs_f64() < 4.90 {
        let core_x = rng.gen_range(0..width);
        let core_y = rng.gen_range(0..height);
        let radius = rng.gen_range(1..=max_radius);

        if grid[core_y][core_x] == '#' {
            skipped_simulations += 1;
            continue;
        }

        let key = core_x * 10000 + core_y * 100 + radius;
        if tried.contains(&key) {
            skipped_simulations += 1;
            continue;
        }
        tried.insert(key);

        let core_bfs = bfs(&grid, (core_x, core_y), 1, 1);
        let mut boundary_cells = vec![];
        for (&cell, &dist) in &core_bfs {
            if dist == radius as i32 {
                if let Some(&burn_time) = dists.get(&cell) {
                    boundary_cells.push((burn_time, cell));
                }
            }
        }
        boundary_cells.sort();

        let mut t = 0;
        let mut valid = true;
        for &(_, (x, y)) in &boundary_cells {
            let burn_time = *dists.get(&(x, y)).unwrap_or(&i32::MAX);
            if burn_time <= t {
                valid = false;
                break;
            }
            t += match grid[y][x] {
                '.' => tree_treatment_duration,
                'X' => house_treatment_duration,
                _ => panic!("Invalid cell"),
            };
        }

        simulations += 1;
        if !valid {
            continue;
        }

        let mut score = 0;
        let fire_dist = *core_bfs
            .get(&(fire_start_x, fire_start_y))
            .unwrap_or(&i32::MAX);
        for (&(x, y), &d) in &core_bfs {
            let val = match grid[y][x] {
                '.' => tree_value,
                'X' => house_value,
                _ => 0,
            };
            if (fire_dist < radius as i32 && d > radius as i32)
                || (fire_dist >= radius as i32 && d < radius as i32)
            {
                score += val;
            }
        }

        if score > best_score {
            best_score = score;
            best_boundary = boundary_cells.iter().map(|&(_, coord)| coord).collect();
        }
    }

    eprintln!(
        "{} simulations + {} skipped",
        simulations, skipped_simulations
    );
    eprintln!("{} best_boundary_score", best_score);

    // Game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let cooldown = parse_input!(input_line, i32);

        for _ in 0..height {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
        }

        if cooldown > 0 || best_boundary.is_empty() {
            println!("WAIT");
        } else {
            let (x, y) = best_boundary.remove(0);
            println!("{} {}", x, y);
        }
    }
}
