use rand::{thread_rng, Rng};

#[derive(Clone, PartialEq)]
pub struct CellData {
    pub cell_type: CellType,
    pub open: bool,
}

#[derive(Clone, PartialEq)]
pub enum CellType {
    Blank,
    Number,
    Bomb,
}

pub fn setup_game(x_axis: usize, y_axis: usize, mut no_of_bombs: usize) -> Vec<Vec<CellData>> {
    let mut rng = thread_rng();
    let mut game_setup = vec![];
    for _ in 0..x_axis {
        let mut current_x = vec![];
        for _ in 0..y_axis {
            let this_cell_type = if no_of_bombs > 0 {
                match rng.gen_range(0..=1) {
                    0 => CellType::Blank,
                    _ => CellType::Bomb,
                }
            } else {
                CellType::Blank
            };
            let this_cell = CellData {
                cell_type: this_cell_type,
                open: false,
            };
            current_x.push(this_cell);
        }
        game_setup.push(current_x);
    }
    game_setup
}
