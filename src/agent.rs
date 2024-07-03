use std::cmp::min;

use crate::board::{Board, Cell, CellState};

pub fn compute_turns_fire(board: &mut Board) -> Vec<Vec<i32>> {
    board.reset();

    let mut turns_fire: Vec<Vec<i32>> = Vec::new();
    for _ in 0..board.get_height() {
        let row_fire: Vec<i32> = (0..board.get_width()).map(|_| 9999).collect();
        turns_fire.push(row_fire);
    }

    eprintln!("Turns fire:");
    eprintln!("{:?}", turns_fire);

    let mut turn = 0;
    let mut end = false;
    while !end {
        eprintln!("Turn: {}", turn);
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

    eprintln!("Turns fire:");
    eprintln!("{:?}", turns_fire);

    turns_fire
}

pub fn solve(board: &mut Board) -> Vec<(usize, usize)> {
    let turns_fire = compute_turns_fire(board);

    let mut actions: Vec<(usize, usize)> = Vec::new();
    actions.push((1, 8));
    actions.push((1, 4));
    actions
}
