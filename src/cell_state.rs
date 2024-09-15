use rand::{thread_rng, Rng};

#[derive(Clone, PartialEq)]
pub struct CellState {
    pub cell_type: CellType,
    pub open: bool,
    pub number: usize,
    pub content: String,
}

#[derive(Clone, PartialEq)]
pub enum CellType {
    Number { local_bombs: usize },
    Bomb,
}

pub fn setup_game(x_axis: usize, y_axis: usize, mut no_of_bombs: usize) -> Vec<Vec<CellState>> {
    let mut rng = thread_rng();
    let mut game_setup = vec![];
    for _ in 0..x_axis {
        let mut current_x = vec![];
        for _ in 0..y_axis {
            let this_cell_type = if no_of_bombs > 0 && rng.gen_range(0..=1) == 1 {
                no_of_bombs -= 1;
                CellType::Bomb
            } else {
                CellType::Number { local_bombs: 0 }
            };
            let this_cell = CellState {
                cell_type: this_cell_type,
                open: false,
                number: 0,
                content: String::new(),
            };
            current_x.push(this_cell);
        }

        game_setup.push(current_x);
    }
    game_setup
}
