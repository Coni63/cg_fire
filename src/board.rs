pub enum CellType {
    Tree,
    House,
    Empty,
}

#[derive(PartialEq, Clone, Copy)]
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
    initial_value: i32,
    value: i32,
    state: CellState,
    initial_state: CellState,
    timer: i32,
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
            initial_value: value,
            value,
            state,
            initial_state: state,
            timer: 0,
        }
    }

    pub fn set_state(&mut self, state: CellState) {
        match state {
            CellState::Safe => {
                self.state = CellState::Safe;
                self.timer = 0;
                self.value = 0;
            }
            CellState::OnFire => {
                if self.state == CellState::Unsafe {
                    self.state = CellState::OnFire;
                    self.timer = self.fire_duration;
                    self.value = 0;
                }
            }
            CellState::Cutting => {
                self.state = CellState::Cutting;
                self.timer = self.cut_duration;
                self.value = 0;
            }
            CellState::Unsafe => {
                self.state = CellState::Unsafe;
                self.timer = 0;
            }
            CellState::Burnt => {
                self.state = CellState::Burnt;
                self.timer = 0;
                self.value = 0;
            }
        }
    }

    pub fn reset(&mut self) {
        self.state = self.initial_state;
        self.value = self.initial_value;
        self.timer = 0;
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

    pub fn reset(&mut self) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                cell.reset();
            }
        }
        self.cooldown = 0;
    }

    pub fn step(&mut self) -> bool {
        let mut count_fire = 0;
        let mut cells_to_propagate: Vec<(usize, usize)> = Vec::new();

        for row in 0..self.height as usize {
            for col in 0..self.width as usize {
                let cell = &mut self.cells[row][col];
                match cell.state {
                    CellState::OnFire => {
                        cell.timer -= 1;
                        count_fire += 1;
                        if cell.timer == 0 {
                            cell.set_state(CellState::Burnt);
                            cells_to_propagate.push((row, col));
                        }
                    }
                    CellState::Cutting => {
                        cell.timer -= 1;
                        self.cooldown = cell.timer;
                        if cell.timer == 0 {
                            cell.set_state(CellState::Safe);
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

    pub fn can_cut(&self) -> bool {
        self.cooldown == 0
    }

    pub fn cut(&mut self, row: usize, col: usize) {
        let cell = &mut self.cells[row][col];
        if cell.state == CellState::Unsafe {
            cell.state = CellState::Cutting;
            cell.value = 0;
            cell.timer = cell.cut_duration;
            self.cooldown = cell.cut_duration;
        }
    }

    fn propagate_fire(&mut self, row: usize, col: usize) {
        self.cells[row - 1][col].set_state(CellState::OnFire);
        self.cells[row + 1][col].set_state(CellState::OnFire);
        self.cells[row][col - 1].set_state(CellState::OnFire);
        self.cells[row][col + 1].set_state(CellState::OnFire);
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
                    CellState::OnFire => print!("{}", cell.timer),
                    CellState::Unsafe => print!("."),
                    CellState::Burnt => print!("#"),
                    CellState::Cutting => print!("C"),
                }
            }
            println!();
        }
    }

    pub fn describe(&self) {
        println!("Cooldown: {}", self.cooldown);
    }

    pub fn score(&self) -> i32 {
        let mut score = 0;
        for row in self.cells.iter() {
            for cell in row.iter() {
                if cell.state == CellState::Unsafe {
                    score += cell.value;
                }
            }
        }
        score
    }
}
