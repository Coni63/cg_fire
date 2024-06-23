pub enum CellType {
    Tree,
    House,
    Empty,
}

pub enum CellState {
    Safe,
    OnFire,
    Unsafe,
    Burnt,
}

pub struct Cell {
    cell_type: CellType,
    fire_duration: i32,
    cut_duration: i32,
    value: i32,
    state: CellState,
}

impl Cell {
    pub fn new(cell_type: CellType, fire_duration: i32, cut_duration: i32, value: i32) -> Cell {
        let state = match cell_type {
            CellType::Tree => CellState::Unsafe,
            CellType::House => CellState::Unsafe,
            CellType::Empty => CellState::Safe,
        };

        Cell {
            cell_type,
            fire_duration,
            cut_duration,
            value,
            state,
        }
    }

    pub fn set_safe(&mut self) {
        self.cell_type = CellType::Empty;
        self.value = 0;
    }

    pub fn set_on_fire(&mut self, fire_duration: i32) {
        self.fire_duration = fire_duration;
        self.value = 0;
        self.state = CellState::OnFire;
    }
}

pub struct Board {
    width: i32,
    height: i32,
    fire_start_x: i32,
    fire_start_y: i32,
    cells: Vec<Vec<Cell>>,
    cooldown: i32,
}

impl Board {
    pub fn new(
        width: i32,
        height: i32,
        fire_start_x: i32,
        fire_start_y: i32,
        cells: Vec<Vec<Cell>>,
        cooldown: i32,
    ) -> Board {
        Board {
            width,
            height,
            fire_start_x,
            fire_start_y,
            cells,
            cooldown,
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[x][y]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[x][y] = cell;
    }

    pub fn show_values(&self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                print!("{:>4} ", cell.value);
            }
            println!();
        }
    }

    pub fn show_types(&self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                match cell.cell_type {
                    CellType::Tree => print!("."),
                    CellType::House => print!("X"),
                    CellType::Empty => print!("#"),
                }
            }
            println!();
        }
    }

    pub fn show_fire(&self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                match cell.state {
                    CellState::Safe => print!("#"),
                    CellState::OnFire => print!("F"),
                    CellState::Unsafe => print!("."),
                    CellState::Burnt => print!("#"),
                }
            }
            println!();
        }
    }
}
