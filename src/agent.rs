use core::time;
use std::{
    cell,
    cmp::min,
    collections::{HashMap, HashSet, VecDeque},
};

use crate::board::{Board, Cell, CellState};

#[derive(Debug, Clone)]
struct Section {
    pub sections: Vec<(i32, i32)>,
    pub time_to_cut: i32,
    pub time_to_reach: i32,
}

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

fn analyse_front(front_fire: HashMap<i32, Vec<Vec<(i32, i32)>>>, board: &Board) -> Vec<Section> {
    let mut options = Vec::new();

    for (timestamp, fronts) in front_fire.iter() {
        for front in fronts.iter() {
            let mut time_to_cut = 1;
            for (r, c) in front.iter().skip(1) {
                time_to_cut += board.get_cell(r, c).get_cut_duration();
            }

            eprintln!(
                "Time to cut: {} vs Time_to_reach: {}",
                time_to_cut, timestamp
            );

            if time_to_cut < *timestamp {
                options.push(Section {
                    sections: front.clone(),
                    time_to_cut,
                    time_to_reach: *timestamp,
                });
            }
        }
    }

    options
}

// Fonction récursive pour générer les combinaisons de sections
fn recursive_generate(
    sections: &[Section],
    current_combination: &mut Vec<Section>,
    current_time: i32,
    result: &mut Vec<Vec<Section>>,
) {
    if current_combination.len() >= 2 {
        return;
    }

    for (index, section) in sections.iter().enumerate() {
        let new_time = current_time + section.time_to_cut;

        if new_time < section.time_to_reach {
            current_combination.push(section.clone());
            result.push(current_combination.clone());
            recursive_generate(
                &sections[index + 1..],
                current_combination,
                new_time,
                result,
            );
            current_combination.pop();
        }
    }
}

fn find_combinations(options: &mut Vec<Section>, board: &mut Board) -> Vec<(i32, i32)> {
    let mut best_score = 0;
    let mut best_option = Vec::new();
    // let timer = std::time::Instant::now();
    // while timer.elapsed().as_millis() < 4500 {

    // }

    options.sort_by_key(|option| option.time_to_reach * 1000 + option.time_to_cut);

    let mut result: Vec<Vec<Section>> = Vec::new();
    let mut current_combination: Vec<Section> = Vec::new();
    recursive_generate(options, &mut current_combination, 0, &mut result);

    eprintln!("{} Combinations", result.len());
    // eprintln!("{:?}", result);

    for all_sections in result.iter() {
        let actions = all_sections
            .iter()
            .flat_map(|section| section.sections.clone())
            .collect();

        let score = evaluate_option(&actions, board);
        // eprintln!("Score: {} - best {} - {:?}", score, best_score, best_option);
        if score > best_score {
            best_score = score;
            best_option = actions.clone();
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

    let mut options = analyse_front(fire_front, board);
    eprintln!("{} Options", options.len());

    find_combinations(&mut options, board)
}
