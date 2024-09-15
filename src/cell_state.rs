use rand::{rngs::ThreadRng, thread_rng, Rng};

#[derive(Clone, PartialEq, Default)]
pub struct CellState {
    pub cell_type: CellType,
    pub open: bool,
    pub number: usize,
}

#[derive(Clone, PartialEq)]
pub enum CellType {
    Number { local_bombs: usize },
    Bomb,
}

fn set_random_cell_to_mine(
    rng: &mut ThreadRng,
    x_axis: usize,
    y_axis: usize,
    game_setup: &mut Vec<Vec<CellState>>,
) {
    let rand_x = rng.gen_range(0..x_axis);
    let rand_y = rng.gen_range(0..y_axis);

    if let CellType::Number { local_bombs: 0 } = game_setup[rand_x][rand_y].cell_type {
        game_setup[rand_x][rand_y].cell_type = CellType::Bomb;
    } else {
        set_random_cell_to_mine(rng, x_axis, y_axis, game_setup);
    }
}

impl Default for CellType {
    fn default() -> Self {
        CellType::Number { local_bombs: 0 }
    }
}

pub fn setup_game(x_axis: usize, y_axis: usize, no_of_bombs: usize) -> Vec<Vec<CellState>> {
    let mut game_setup = vec![];
    for _ in 0..x_axis {
        game_setup.push(vec![CellState::default(); y_axis]);
    }

    let mut rng = thread_rng();
    for _ in 0..no_of_bombs {
        set_random_cell_to_mine(&mut rng, x_axis, y_axis, &mut game_setup)
    }

    game_setup
}
