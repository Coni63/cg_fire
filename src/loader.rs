use std::io;

use crate::board::{Board, Cell, CellState, CellType};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Read the constant data of the map before the main loop, then read the state of the fire and give an action at each turn
 **/
pub fn load_input() -> Board {
    let mut board: Vec<Vec<Cell>> = Vec::new();

    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let tree_treatment_duration = parse_input!(inputs[0], i32); // cooldown for cutting a "tree" cell
    let tree_fire_duration = parse_input!(inputs[1], i32); // number of turns for the fire to propagate on adjacent cells from a "tree" cell
    let tree_value = parse_input!(inputs[2], i32); // value lost if a "tree" cell is burnt or cut

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let house_treatment_duration = parse_input!(inputs[0], i32); // cooldown for cutting a "house" cell
    let house_fire_duration = parse_input!(inputs[1], i32); // number of turns for the fire to propagate on adjacent cells from a "house" cell
    let house_value = parse_input!(inputs[2], i32); // value lost if a "house" cell is burnt or cut

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let width = parse_input!(inputs[0], i32); // number of columns in the grid
    let height = parse_input!(inputs[1], i32); // number of rows in the grid

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let fire_start_x = parse_input!(inputs[0], i32); // column where the fire starts
    let fire_start_y = parse_input!(inputs[1], i32); // row where the fire starts
    for _ in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let grid_line = input_line.trim().to_string();

        let mut row: Vec<Cell> = Vec::new();
        for c in grid_line.chars() {
            let cell = match c {
                '.' => Cell::new(
                    CellType::Tree,
                    tree_fire_duration,
                    tree_treatment_duration,
                    tree_value,
                ),
                'X' => Cell::new(
                    CellType::House,
                    house_fire_duration,
                    house_treatment_duration,
                    house_value,
                ),
                '#' => Cell::new(CellType::Empty, 0, 0, 0),
                _ => panic!("Invalid cell type"),
            };
            row.push(cell);
        }
        board.push(row);
    }

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let cooldown = parse_input!(input_line, i32); // number of turns remaining before you can cut a new cell

    for i in 0..height as usize {
        let mut inputs = String::new();
        io::stdin().read_line(&mut inputs).unwrap();
        for (j, value) in inputs.split_whitespace().enumerate() {
            let fire_progress = parse_input!(value, i32);
            match fire_progress {
                -2 => {
                    board[i][j].set_state(CellState::Safe);
                }
                -1 => (),
                _ => {
                    // TODO: set fire duration
                    board[i][j].set_state(CellState::OnFire);
                }
            }
        }
    }

    Board::new(width, height, fire_start_x, fire_start_y, board, cooldown)
}
