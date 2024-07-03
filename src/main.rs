mod agent;
mod board;
mod loader;

fn main() {
    let mut board = loader::load_input();

    // board.show_values();
    // board.show_types();
    // board.show_fire();

    board.reset();

    let actions: Vec<(usize, usize)> = agent::solve(&mut board);

    let mut turn = 0;
    let mut idx_action = 0;
    let mut end = false;
    while !end {
        if board.can_cut() && (idx_action < actions.len()) {
            let (row, col) = actions[idx_action];
            board.cut(row, col);
            idx_action += 1;
        }
        end = board.step();
        turn += 1;
        println!("Turn: {}", turn);
        board.describe();
        board.show_fire();
    }
}
