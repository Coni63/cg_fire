#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Tree,
    TreeBurning(i32),
    TreeBurnt,
    TreeCutting(i32),
    TreeCut,
    House,
    HouseBurning(i32),
    HouseBurnt,
    HouseCutting(i32),
    HouseCut,
    Empty,
}

pub struct Board {
    width: usize,
    height: usize,

    tree_cut_duration: i32,
    tree_fire_duration: i32,
    tree_value: i32,

    house_cut_duration: i32,
    house_fire_duration: i32,
    house_value: i32,

    fire_start_x: usize,
    fire_start_y: usize,

    cells: [Cell; 2500],

    cooldown: i32,
    fires: Vec<usize>,
    cutting: usize,
}

impl Board {
    pub fn new(
        width: usize,
        height: usize,

        tree_cut_duration: i32,
        tree_fire_duration: i32,
        tree_value: i32,
        house_cut_duration: i32,
        house_fire_duration: i32,
        house_value: i32,

        fire_start_x: usize,
        fire_start_y: usize,

        cells: [Cell; 2500],

        cooldown: i32,
    ) -> Board {
        let mut board = Board {
            width,
            height,

            tree_cut_duration,
            tree_fire_duration,
            tree_value,
            house_cut_duration,
            house_fire_duration,
            house_value,

            fire_start_x,
            fire_start_y,

            cells,

            cooldown,
            fires: Vec::new(),
            cutting: 0,
        };

        let idx = fire_start_y * 50 + fire_start_x;
        board.set_fire(idx);
        board
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_cell(&self, idx: usize) -> &Cell {
        &self.cells[idx]
    }

    pub fn reset(&mut self) {
        for row in 1..self.height - 1 {
            for col in 1..self.width - 1 {
                let idx = row * 50 + col;
                match self.cells[idx] {
                    Cell::TreeBurning(_) => self.cells[idx] = Cell::Tree,
                    Cell::HouseBurning(_) => self.cells[idx] = Cell::House,
                    Cell::HouseBurnt => self.cells[idx] = Cell::House,
                    Cell::TreeBurnt => self.cells[idx] = Cell::Tree,
                    Cell::TreeCutting(_) => self.cells[idx] = Cell::Tree,
                    Cell::HouseCutting(_) => self.cells[idx] = Cell::House,
                    Cell::HouseCut => self.cells[idx] = Cell::House,
                    Cell::TreeCut => self.cells[idx] = Cell::Tree,
                    _ => {}
                }
            }
        }

        self.fires.clear();
        let idx = self.fire_start_y * 50 + self.fire_start_x;
        self.set_fire(idx);
    }

    pub fn step(&mut self) -> bool {
        let mut curr_fires = self.fires.clone();
        self.fires.clear();

        for idx in curr_fires.iter() {
            match self.cells[*idx] {
                Cell::TreeBurning(1) => {
                    self.cells[*idx] = Cell::TreeBurnt;
                    self.propagate_fire(*idx);
                }
                Cell::HouseBurning(1) => {
                    self.cells[*idx] = Cell::HouseBurnt;
                    self.propagate_fire(*idx);
                }
                Cell::TreeBurning(turns) => {
                    self.cells[*idx] = Cell::TreeBurning(turns - 1);
                    self.fires.push(*idx);
                }
                Cell::HouseBurning(turns) => {
                    self.cells[*idx] = Cell::TreeBurning(turns - 1);
                    self.fires.push(*idx);
                }
                _ => {}
            }
        }

        self.fires.is_empty()
    }

    pub fn can_act(&self) -> bool {
        self.cooldown == 0
    }

    pub fn cut(&mut self, idx: usize) {
        match self.cells[idx] {
            Cell::Tree => {
                self.cells[idx] = Cell::TreeCutting(self.tree_cut_duration);
                self.cooldown = self.tree_cut_duration;
            }
            Cell::House => {
                self.cells[idx] = Cell::HouseCutting(self.house_cut_duration);
                self.cooldown = self.house_cut_duration;
            }
            _ => {}
        }
    }

    pub fn get_value(&self, idx: usize) -> i32 {
        match self.cells[idx] {
            Cell::Tree => self.tree_value,
            Cell::House => self.house_value,
            _ => 0,
        }
    }

    fn propagate_fire(&mut self, idx: usize) {
        self.set_fire(idx - 50);
        self.set_fire(idx + 50);
        self.set_fire(idx - 1);
        self.set_fire(idx + 1);
    }

    fn set_fire(&mut self, idx: usize) {
        match self.cells[idx] {
            Cell::Tree => {
                self.cells[idx] = Cell::TreeBurning(self.tree_fire_duration);
                self.fires.push(idx);
            }
            Cell::House => {
                self.cells[idx] = Cell::HouseBurning(self.house_fire_duration);
                self.fires.push(idx);
            }
            _ => {}
        }
    }

    pub fn show_values(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = row * 50 + col;
                eprint!("{:>4} ", self.get_value(idx));
            }
            eprintln!();
        }
    }

    pub fn show_state(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = row * 50 + col;
                match self.cells[idx] {
                    Cell::Tree => eprint!("."),
                    Cell::TreeBurning(_) => eprint!("F"),
                    Cell::TreeBurnt => eprint!("#"),
                    Cell::TreeCutting(_) => eprint!("+"),
                    Cell::TreeCut => eprint!("#"),

                    Cell::House => eprint!("X"),
                    Cell::HouseBurning(_) => eprint!("F"),
                    Cell::HouseBurnt => eprint!("#"),
                    Cell::HouseCutting(_) => eprint!("+"),
                    Cell::HouseCut => eprint!("#"),

                    Cell::Empty => eprint!("#"),
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
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = row * 50 + col;
                score += self.get_value(idx);
            }
            eprintln!();
        }
        score
    }
}
