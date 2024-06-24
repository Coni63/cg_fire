mod board;
mod loarder;

fn main() {
    let mut board = loarder::load_input();

    // board.show_values();
    // board.show_types();
    board.show_fire();

    let mut turn = 1;
    let mut end = false;
    while !end {
        end = board.step();
        turn += 1;
        println!("Turn: {}", turn);
        board.show_fire();
    }
}
