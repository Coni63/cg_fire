#[derive(PartialEq)]
pub enum CellType {
    Tree,
    House,
    Empty,
}

#[derive(PartialEq)]
pub enum CellState {
    Safe,
    OnFire,
    Cutting,
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

    pub fn set_on_fire(&mut self) {
        if self.state != CellState::Unsafe {
            return;
        }
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

    pub fn step(&mut self) -> bool {
        let mut count_fire = 0;
        let mut cells_to_propagate: Vec<(usize, usize)> = Vec::new();

        for row in 0..self.height as usize {
            for col in 0..self.width as usize {
                let cell = &mut self.cells[row][col];
                match cell.state {
                    CellState::OnFire => {
                        cell.fire_duration -= 1;
                        count_fire += 1;
                        if cell.fire_duration == 0 {
                            cell.state = CellState::Burnt;
                            cells_to_propagate.push((row, col));
                        }
                    }
                    CellState::Cutting => {
                        cell.cut_duration -= 1;
                        if cell.cut_duration == 0 {
                            cell.set_safe();
                        }
                    }
                    _ => {}
                }
            }
        }

        for (row, col) in cells_to_propagate.iter() {
            self.propagate_fire(*row, *col);
        }
        count_fire == 0
    }

    pub fn cut(&mut self, row: usize, col: usize) {
        let cell = &mut self.cells[row][col];
        if cell.state == CellState::Unsafe {
            cell.state = CellState::Cutting;
            cell.value = 0;
            cell.cut_duration -= 1;
            self.cooldown = cell.cut_duration;
        }
    }

    fn propagate_fire(&mut self, row: usize, col: usize) {
        self.cells[row - 1][col].set_on_fire();
        self.cells[row + 1][col].set_on_fire();
        self.cells[row][col - 1].set_on_fire();
        self.cells[row][col + 1].set_on_fire();
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
                    CellState::OnFire => print!("{}", cell.fire_duration),
                    CellState::Unsafe => print!("."),
                    CellState::Burnt => print!("#"),
                    CellState::Cutting => print!("C"),
                }
            }
            println!();
        }
    }
}
