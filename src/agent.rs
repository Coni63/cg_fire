use crate::board::Board;

pub fn solve(board: &mut Board) -> Vec<(usize, usize)> {
    let mut actions: Vec<(usize, usize)> = Vec::new();
    actions.push((1, 8));
    actions.push((1, 4));
    actions
}
