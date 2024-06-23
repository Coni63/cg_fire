mod board;
mod loarder;

fn main() {
    let mut board = loarder::load_input();

    board.show_values();
    board.show_types();
    board.show_fire();
}
