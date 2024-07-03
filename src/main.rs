mod agent;
mod board;
mod loader;

fn main() {
    let mut board = loader::load_input();

    // board.show_values();
    // board.show_types();
    // board.show_fire();

    let start_time = std::time::Instant::now();
    let actions = agent::solve(&mut board);
    eprintln!("Time: {:?}", start_time.elapsed());
    eprintln!("Actions: {:?}", actions);

    board.reset();

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
        eprintln!("Turn: {}", turn);
        board.describe();
        board.show_fire();
    }
}

/*

MAIN FOR CG

fn main() {
    let mut board = load_input();

    let start_time = std::time::Instant::now();
    let actions = solve(&mut board);

    eprintln!("{:?}", actions);

    board.reset();
    let mut idx_action = 0;
    let mut cooldown = 0;
    // game loop
    loop {
        if cooldown == 0 {
            if idx_action < actions.len() {
                let (row, col) = actions[idx_action];
                idx_action += 1;
                println!("{} {}", col, row)
            } else {
                println!("WAIT");
            }
        } else {
            println!("WAIT");
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        cooldown = parse_input!(input_line, i32); // number of turns remaining before you can cut a new cell
        for i in 0..board.get_height() as usize {
            let mut inputs = String::new();
            io::stdin().read_line(&mut inputs).unwrap();
            for j in inputs.split_whitespace() {
                let fire_progress = parse_input!(j, i32);
            }
        }
    }
}

*/
