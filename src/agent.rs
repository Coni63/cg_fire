use std::{
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
                if let CellState::OnFire = board.get_cell(row, col).get_state() {
                    turns_fire[row as usize][col as usize] =
                        min(turns_fire[row as usize][col as usize], turn);
                }
            }
        }
    }

    board.reset();

    turns_fire
}

fn compute_borders(turns: &Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
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
    let mut length: HashMap<i32, Vec<i32>> = HashMap::new();
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
            let mut count = 0;

            queue.push_back((row as i32, col as i32));

            while !queue.is_empty() {
                let (r, c) = queue.pop_front().unwrap();

                if visited.contains(&(r * 100 + c)) {
                    continue;
                }

                if *target_value == turns[r as usize][c as usize] {
                    count += 1;
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

pub fn solve(board: &mut Board) -> Vec<(usize, usize)> {
    let turns_fire = compute_turns_fire(board);
    let fire_front = compute_borders(&turns_fire);

    let mut actions: Vec<(usize, usize)> = Vec::new();
    actions.push((1, 8));
    actions.push((1, 4));
    actions
}
