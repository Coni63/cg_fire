use std::collections::VecDeque;

use crate::board::{Board, Cell};

#[derive(Debug, Clone)]
struct Section {
    pub sections: Vec<usize>,
    pub time_to_cut: i32,
    pub time_to_reach: i32,
    pub time_to_save: i32,
}

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
                        eprintln!("New: {}", new_idx);
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

    eprintln!("{:?} City Sections", city_sections);

    let mut sections = Vec::new();
    sections.extend(front_sections);
    sections.extend(city_sections);

    sections.sort_by_key(|option| option.time_to_reach * 1000 + option.time_to_save);

    sections
}

// Fonction récursive pour générer les combinaisons de sections
// fn recursive_generate(
//     sections: &[Section],
//     current_combination: &mut Vec<Section>,
//     current_time: i32,
//     result: &mut Vec<Vec<Section>>,
// ) {
//     if current_combination.len() >= 2 {
//         return;
//     }

//     for (index, section) in sections.iter().enumerate() {
//         let new_time = current_time + section.time_to_cut;

//         if new_time < section.time_to_reach {
//             current_combination.push(section.clone());
//             result.push(current_combination.clone());
//             recursive_generate(
//                 &sections[index + 1..],
//                 current_combination,
//                 new_time,
//                 result,
//             );
//             current_combination.pop();
//         }
//     }
// }

// fn find_combinations(options: &[Section], board: &mut Board) -> Vec<usize> {
//     let mut best_score = 0;
//     let mut best_option = Vec::new();

//     let mut result: Vec<Vec<Section>> = Vec::new();
//     let mut current_combination: Vec<Section> = Vec::new();
//     recursive_generate(options, &mut current_combination, 0, &mut result);

//     eprintln!("{} Combinations", result.len());
//     // eprintln!("{:?}", result);

//     for all_sections in result.iter() {
//         let actions: Vec<usize> = all_sections
//             .iter()
//             .flat_map(|section| section.sections.clone())
//             .collect();

//         let score = evaluate_option(&actions, board);
//         // eprintln!("Score: {} - best {} - {:?}", score, best_score, best_option);
//         if score > best_score {
//             best_score = score;
//             best_option = actions.clone();
//         }
//     }

//     best_option
// }

fn find_combinations(options: &[Section], board: &mut Board) -> Vec<usize> {
    let mut best_score = 0;
    let mut best_option = Vec::new();

    for (index, section) in options.iter().enumerate() {
        let actions: Vec<usize> = section.sections.clone();
        let score = evaluate_option(&actions, board);
        eprintln!("Score: {} - best {} - {:?}", score, best_score, best_option);
        if score > best_score {
            best_score = score;
            best_option = actions.clone();
        }
    }

    best_option
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
    eprintln!("{} Sections", sections.len());

    find_combinations(&sections, board)
}
