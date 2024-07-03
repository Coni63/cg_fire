pub enum CellType {
    Tree,
    House,
    Empty,
}

#[derive(PartialEq, Clone, Copy, Debug)]
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

    pub fn set_initial_fire(&mut self) {
        self.state = CellState::OnFire;
        self.initial_state = CellState::OnFire;
        self.value = 0;
        self.initial_value = 0;
        self.timer = self.fire_duration;
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
        self.timer = if self.state == CellState::OnFire {
            self.fire_duration
        } else {
            0
        };
    }

    pub fn get_state(&self) -> CellState {
        self.state
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
        let mut board = Board {
            width,
            height,
            fire_start_x,
            fire_start_y,
            cells,
            cooldown,
        };

        board.cells[fire_start_y as usize][fire_start_x as usize].set_initial_fire();
        board
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_cell(&self, row: i32, col: i32) -> &Cell {
        &self.cells[row as usize][col as usize]
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
                eprint!("{:>4} ", cell.value);
            }
            eprintln!();
        }
    }

    pub fn show_types(&self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                match cell.cell_type {
                    CellType::Tree => eprint!("."),
                    CellType::House => eprint!("X"),
                    CellType::Empty => eprint!("#"),
                }
            }
            eprintln!();
        }
    }

    pub fn show_fire(&self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                match cell.state {
                    CellState::Safe => eprint!("#"),
                    CellState::OnFire => eprint!("{}", cell.timer),
                    CellState::Unsafe => eprint!("."),
                    CellState::Burnt => eprint!("#"),
                    CellState::Cutting => eprint!("C"),
                }
            }
            eprintln!();
        }
    }

    pub fn describe(&self) {
        eprintln!("Cooldown: {} - Score {}", self.cooldown, self.score());
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
