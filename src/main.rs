mod agent;
mod board;

use std::io;

use crate::agent::solve;
use crate::board::{Board, Cell};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Read the constant data of the map before the main loop, then read the state of the fire and give an action at each turn
 **/
pub fn load_turn_input(height: usize) {
    // per turn input
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    for _ in 0..height {
        io::stdin().read_line(&mut input_line).unwrap();
    }
}

pub fn load_input() -> Board {
    let mut cells: [Cell; 2500] = [Cell::Empty; 2500];

    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let tree_cut_duration = parse_input!(inputs[0], i32); // cooldown for cutting a "tree" cell
    let tree_fire_duration = parse_input!(inputs[1], i32); // number of turns for the fire to propagate on adjacent cells from a "tree" cell
    let tree_value = parse_input!(inputs[2], i32); // value lost if a "tree" cell is burnt or cut

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let house_cut_duration = parse_input!(inputs[0], i32); // cooldown for cutting a "house" cell
    let house_fire_duration = parse_input!(inputs[1], i32); // number of turns for the fire to propagate on adjacent cells from a "house" cell
    let house_value = parse_input!(inputs[2], i32); // value lost if a "house" cell is burnt or cut

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let width = parse_input!(inputs[0], usize); // number of columns in the grid
    let height = parse_input!(inputs[1], usize); // number of rows in the grid

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let fire_start_x = parse_input!(inputs[0], usize); // column where the fire starts
    let fire_start_y = parse_input!(inputs[1], usize); // row where the fire starts
    let fire_start = fire_start_y * 50 + fire_start_x;

    // Read the map
    for row in 0..height {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let grid_line = input_line.trim().to_string();
        for (col, c) in grid_line.chars().enumerate() {
            cells[row * 50 + col] = match c {
                '.' => Cell::Tree,
                'X' => Cell::House,
                '#' => Cell::Empty,
                _ => panic!("Invalid cell type"),
            };
        }
    }

    load_turn_input(height);

    Board::new(
        width,
        height,
        tree_cut_duration,
        tree_fire_duration,
        tree_value,
        house_cut_duration,
        house_fire_duration,
        house_value,
        fire_start,
        cells,
    )
}

fn main() {
    let mut board = load_input();

    let start_time = std::time::Instant::now();
    let actions = solve(&mut board);
    eprintln!("Time: {:?}", start_time.elapsed());
    eprintln!("Actions: {:?}", actions);

    let mut turn = 1;
    let mut idx_action = 0;
    let mut end = false;
    while !end {
        if board.can_act() && (idx_action < actions.len()) {
            board.cut(actions[idx_action]);
            idx_action += 1;
        }
        end = board.step();
        turn += 1;
    }

    println!("{} pts - {} turns", board.score(), turn);
}

/*

MAIN FOR CG

fn main() {
    let mut board = load_input();

    let start_time = std::time::Instant::now();
    let actions = solve(&mut board);

    let mut idx_action = 0;
    let mut cooldown = 0;
    // game loop
    loop {
        if board.can_act() && (idx_action < actions.len()) {
            let idx = actions[idx_action];
            board.cut(idx);
            idx_action += 1;
            println!("{} {}", idx%50, idx/50)
        } else {
            println!("WAIT");
        }

        board.step();

        load_turn_input(board.get_height());
    }
}

*/
