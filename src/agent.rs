use core::time;
use std::{
    cell,
    cmp::min,
    collections::{HashMap, HashSet, VecDeque},
};

use crate::board::{Board, Cell, CellState};

pub fn compute_turns_fire(board: &mut Board) -> Vec<Vec<i32>> {
    board.reset();

    let mut turns_fire: Vec<Vec<i32>> = Vec::new();
    for _ in 0..board.get_height() {
        let row_fire: Vec<i32> = (0..board.get_width()).map(|_| 9999).collect();
        turns_fire.push(row_fire);
    }

    let mut turn = 0;
    let mut end = false;
    while !end {
        end = board.step();
        turn += 1;
        for row in 0..board.get_height() {
            for col in 0..board.get_width() {
                if let CellState::OnFire = board.get_cell(&row, &col).get_state() {
                    turns_fire[row as usize][col as usize] =
                        min(turns_fire[row as usize][col as usize], turn);
                }
            }
        }
    }

    board.reset();

    turns_fire
}

fn compute_borders(turns: &Vec<Vec<i32>>) -> HashMap<i32, Vec<Vec<(i32, i32)>>> {
    let offset = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let mut length: HashMap<i32, Vec<Vec<(i32, i32)>>> = HashMap::new();
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    let mut visited: HashSet<i32> = HashSet::new();
    for (row, values) in turns.iter().enumerate() {
        for (col, value) in values.iter().enumerate() {
            if (*value == 0) | (*value == 9999) {
                continue;
            }

            // look for the first non visited cell
            let hash = row as i32 * 100 + col as i32;
            if visited.contains(&hash) {
                continue;
            }

            // start a BFS from this cell
            let target_value = value;
            let mut count = Vec::new();

            queue.push_back((row as i32, col as i32));

            while !queue.is_empty() {
                let (r, c) = queue.pop_front().unwrap();

                if visited.contains(&(r * 100 + c)) {
                    continue;
                }

                if *target_value == turns[r as usize][c as usize] {
                    count.push((r, c));
                    visited.insert(r * 100 + c);

                    for (dr, dc) in offset.iter() {
                        queue.push_back((r + dr, c + dc));
                    }
                }
            }

            length.entry(*target_value).or_default().push(count);
        }
    }
    length
}

fn analyse_front(
    front_fire: HashMap<i32, Vec<Vec<(i32, i32)>>>,
    board: &Board,
) -> Vec<Vec<(i32, i32)>> {
    let mut options = Vec::new();

    for (timestamp, fronts) in front_fire.iter() {
        for front in fronts.iter() {
            let mut time_to_cut = 0;
            for (r, c) in front.iter() {
                time_to_cut += board.get_cell(r, c).get_cut_duration();
            }

            if time_to_cut < *timestamp {
                options.push(front.clone());
            }
        }
    }

    options
}

fn find_combinations(options: Vec<Vec<(i32, i32)>>, board: &mut Board) -> Vec<(i32, i32)> {
    let mut best_score = 0;
    let mut best_option = Vec::new();
    // let timer = std::time::Instant::now();
    // while timer.elapsed().as_millis() < 4500 {

    // }

    for option in options.iter() {
        let score = evaluate_option(option, board);
        // eprintln!("Score: {} - best {} - {:?}", score, best_score, best_option);
        if score > best_score {
            best_score = score;
            best_option = option.clone();
        }
    }

    best_option
}

fn evaluate_option(actions: &Vec<(i32, i32)>, board: &mut Board) -> i32 {
    board.reset();
    let mut idx_action = 0;
    let mut end = false;
    while !end {
        if board.can_cut() && (idx_action < actions.len()) {
            let (row, col) = actions[idx_action];
            board.cut(row, col);
            idx_action += 1;
        }
        end = board.step();
    }

    board.score()
}

pub fn solve(board: &mut Board) -> Vec<(i32, i32)> {
    let turns_fire = compute_turns_fire(board);
    let fire_front = compute_borders(&turns_fire);

    // eprintln!("Fire front: {:?}", fire_front);

    let options = analyse_front(fire_front, board);
    eprintln!("{} Options: {:?}", options.len(), options);

    find_combinations(options, board)
}
