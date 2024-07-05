use rand::{self, Rng};
use std::collections::VecDeque;

use crate::board::{Board, Cell};

#[derive(Debug, Clone)]
struct Section {
    pub sections: Vec<usize>,
    pub time_to_cut: i32,
    pub time_to_reach: i32,
    pub time_to_save: i32,
}

/*
 * Compute the sections of the board that can be cut
 */

fn compute_front_sections(board: &mut Board) -> Vec<Section> {
    let mut sections = Vec::new();
    let mut visited: [bool; 2500] = [false; 2500];
    let offset = [-1, -50, 1, 50, -51, -49, 49, 51];

    let mut end = false;
    let mut turn = 0;
    while !end {
        let fires = board.get_fire();
        for idx in fires.iter() {
            if visited[*idx] {
                continue;
            }

            let mut queue: VecDeque<usize> = VecDeque::new();
            let mut section: Vec<usize> = Vec::new();

            queue.push_back(*idx);
            while let Some(idx) = queue.pop_front() {
                if visited[idx] {
                    continue;
                }
                visited[idx] = true;

                section.push(idx);

                for off in offset.iter() {
                    let new_idx = (idx as i32 + off) as usize;
                    if fires.contains(&new_idx) & !visited[new_idx] {
                        queue.push_back(new_idx);
                    }
                }
            }

            let times: Vec<i32> = section
                .iter()
                .map(|idx| board.get_cut_duration(*idx))
                .collect();
            let time_to_cut = times.iter().sum();
            let time_to_save = time_to_cut - times[0] + 1;
            let section = Section {
                sections: section,
                time_to_cut,
                time_to_reach: turn,
                time_to_save,
            };

            if section.time_to_save <= section.time_to_reach {
                sections.push(section);
            }
        }
        end = board.step();
        turn += 1;
    }
    board.reset();

    sections
}

fn compute_city_sections(board: &mut Board) -> Vec<Section> {
    let offset = [-1, -50, 1, 50];
    let mut visited: [bool; 2500] = [false; 2500];
    let mut sections: Vec<Section> = Vec::new();

    for row in 1..board.get_height() - 1 {
        for col in 1..board.get_width() - 1 {
            let idx = row * 50 + col;
            if visited[idx] {
                continue;
            }

            if let Cell::House = board.get_cell(idx) {
                let mut section: Vec<usize> = Vec::new();
                let mut queue: VecDeque<usize> = VecDeque::new();
                queue.push_back(idx);
                while let Some(idx) = queue.pop_front() {
                    if visited[idx] {
                        continue;
                    }
                    visited[idx] = true;

                    for off in offset.iter() {
                        let new_idx = (idx as i32 + off) as usize;
                        match board.get_cell(new_idx) {
                            Cell::House => {
                                queue.push_back(new_idx);
                            }
                            Cell::Tree => {
                                if !section.contains(&new_idx) {
                                    section.push(new_idx);
                                }
                            }
                            _ => {}
                        }
                    }
                }

                if section.is_empty() {
                    continue;
                }

                section.sort_by_key(|idx| board.get_reached_duration(*idx));

                let times: Vec<i32> = section
                    .iter()
                    .map(|idx| board.get_cut_duration(*idx))
                    .collect();
                let time_to_cut = times.iter().sum();
                let time_to_save = time_to_cut - times[0] + 1;
                let time_to_reach = section
                    .iter()
                    .map(|idx| board.get_reached_duration(*idx))
                    .min()
                    .unwrap_or(0) as i32;

                let section = Section {
                    sections: section,
                    time_to_cut,
                    time_to_reach,
                    time_to_save,
                };

                sections.push(section);
            }
        }
    }

    sections
}

fn compute_sections(board: &mut Board) -> Vec<Section> {
    let front_sections = compute_front_sections(board);
    let city_sections = compute_city_sections(board);

    let mut sections = Vec::new();
    sections.extend(front_sections);
    sections.extend(city_sections);

    sections.sort_by_key(|option| option.time_to_reach * 1000 + option.time_to_save);

    sections
}

/*
 * Find the best combination of sections to cut
 */
fn find_combinations(options: &[Section], board: &mut Board) -> Vec<usize> {
    let mut rng = rand::thread_rng();

    let mut best_score = 0;
    let mut best_option = Vec::new();
    let mut pick_option: Vec<usize> = Vec::new();
    let num_sections = options.len();

    let mut simulation = 0;
    let timer = std::time::Instant::now();
    while timer.elapsed().as_millis() < 900 {
        let mut total_cut_time = 0;
        let mut first_idx = 0;
        pick_option.clear();

        loop {
            first_idx = rng.gen_range(first_idx..num_sections);
            let section = &options[first_idx];
            if section.time_to_reach < total_cut_time {
                break;
            }
            total_cut_time += options[first_idx].time_to_cut;
            pick_option.push(first_idx);
            first_idx += if first_idx < num_sections - 1 { 1 } else { 0 };

            if rng.gen_bool(0.1) {
                break;
            }
        }

        let actions: Vec<usize> = pick_option
            .iter()
            .flat_map(|idx| options[*idx].sections.clone())
            .collect();

        // eprintln!("Actions: {:?}", actions);

        let score = evaluate_option(&actions, board);
        // eprintln!("Score: {}", score);
        if score > best_score {
            best_score = score;
            best_option = pick_option.clone();
        }

        simulation += 1;
    }

    // eprintln!("Simulation: {}", simulation);

    best_option
        .iter()
        .flat_map(|idx| options[*idx].sections.clone())
        .collect()
}

fn evaluate_option(actions: &[usize], board: &mut Board) -> i32 {
    let mut idx_action = 0;
    let mut end = false;
    while !end {
        if board.can_act() && (idx_action < actions.len()) {
            board.cut(actions[idx_action]);
            idx_action += 1;
        }
        end = board.step();
    }

    let score = board.score();
    board.reset();
    score
}

pub fn solve(board: &mut Board) -> Vec<usize> {
    let sections = compute_sections(board);

    find_combinations(&sections, board)
}
