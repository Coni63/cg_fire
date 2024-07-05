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

    fire_start: usize,

    cells: [Cell; 2500],

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

        fire_start: usize,

        cells: [Cell; 2500],
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

            fire_start,

            cells,

            fires: Vec::new(),
            cutting: 0,
        };

        board.set_fire(fire_start);
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
        self.set_fire(self.fire_start);
    }

    pub fn step(&mut self) -> bool {
        let curr_fires = self.fires.clone();
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
                    self.cells[*idx] = Cell::HouseBurning(turns - 1);
                    self.fires.push(*idx);
                }
                _ => {}
            }
        }

        match self.cells[self.cutting] {
            Cell::TreeCutting(1) => {
                self.cells[self.cutting] = Cell::TreeCut;
                self.cutting = 0;
            }
            Cell::HouseCutting(1) => {
                self.cells[self.cutting] = Cell::HouseCut;
                self.cutting = 0;
            }
            Cell::TreeCutting(turns) => {
                self.cells[self.cutting] = Cell::TreeCutting(turns - 1);
            }
            Cell::HouseCutting(turns) => {
                self.cells[self.cutting] = Cell::HouseCutting(turns - 1);
            }
            _ => {}
        }

        self.fires.is_empty()
    }

    pub fn can_act(&self) -> bool {
        if self.cutting == 0 {
            return true;
        }
        match self.cells[self.cutting] {
            Cell::TreeCutting(turns) => turns == 0,
            Cell::HouseCutting(turns) => turns == 0,
            _ => true,
        }
    }

    pub fn cut(&mut self, idx: usize) {
        match self.cells[idx] {
            Cell::Tree => {
                self.cells[idx] = Cell::TreeCutting(self.tree_cut_duration);
                self.cutting = idx;
            }
            Cell::House => {
                self.cells[idx] = Cell::HouseCutting(self.house_cut_duration);
                self.cutting = idx;
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

    pub fn get_fire(&self) -> &Vec<usize> {
        &self.fires
    }

    pub fn get_cut_duration(&self, idx: usize) -> i32 {
        match self.cells[idx] {
            Cell::Tree => self.tree_cut_duration,
            Cell::House => self.house_cut_duration,
            Cell::HouseBurning(_) => self.house_cut_duration,
            Cell::TreeBurning(_) => self.tree_cut_duration,
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
        eprintln!("Score {}", self.score());
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

    pub fn score(&self) -> i32 {
        let mut score = 0;
        for row in 1..self.height - 1 {
            for col in 1..self.width - 1 {
                score += self.get_value(row * 50 + col);
            }
        }
        score
    }
}
