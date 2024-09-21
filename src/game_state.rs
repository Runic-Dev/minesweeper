use leptos::logging::log;
use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::tile_state::{
    TileState,
    TileType::{Bomb, Number},
};

#[derive(Clone)]
pub struct GameState {
    pub grid: Vec<Vec<TileState>>,
    pub game_over: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            grid: setup_game(10, 20, 25),
            game_over: false,
        }
    }
}

pub fn setup_game(x_axis: usize, y_axis: usize, no_of_bombs: usize) -> Vec<Vec<TileState>> {
    let mut game_setup = vec![];
    for _ in 0..x_axis {
        game_setup.push(vec![TileState::default(); y_axis]);
    }

    let mut rng = thread_rng();
    for _ in 0..no_of_bombs {
        set_random_cell_to_mine(&mut rng, x_axis, y_axis, &mut game_setup)
    }

    game_setup
}

fn set_random_cell_to_mine(
    rng: &mut ThreadRng,
    x_axis: usize,
    y_axis: usize,
    game_setup: &mut Vec<Vec<TileState>>,
) {
    let rand_x = rng.gen_range(0..x_axis);
    let rand_y = rng.gen_range(0..y_axis);

    if let Number { local_bombs: _ } = game_setup[rand_x][rand_y].cell_type {
        game_setup[rand_x][rand_y].cell_type = Bomb;
        let neighbours_result = get_neighbours(rand_x, rand_y, x_axis, y_axis);

        match neighbours_result {
            Ok(neighbours) => {
                neighbours.into_iter().for_each(|(x, y)| {
                    if let Number {
                        ref mut local_bombs,
                    } = game_setup[x][y].cell_type
                    {
                        *local_bombs += 1
                    }
                });
            }
            Err(message) => log!("{}", message),
        }
    } else {
        set_random_cell_to_mine(rng, x_axis, y_axis, game_setup);
    }
}

pub fn get_neighbours(
    rand_x: usize,
    rand_y: usize,
    x_len: usize,
    y_len: usize,
) -> Result<Vec<(usize, usize)>, String> {
    match (rand_x, rand_y) {
        (x, y) if x > 0 && x < x_len - 1 && y > 0 && y < y_len - 1 => Ok(vec![
            (x - 1, y),
            (x - 1, y - 1),
            (x - 1, y + 1),
            (x + 1, y),
            (x + 1, y - 1),
            (x + 1, y + 1),
            (x, y - 1),
            (x, y + 1),
        ]),
        (x, y) if x == 0 && y > 0 && y < y_len - 1 => Ok(vec![
            (x + 1, y),
            (x + 1, y - 1),
            (x + 1, y + 1),
            (x, y - 1),
            (x, y + 1),
        ]),
        (x, y) if x == x_len - 1 && y > 0 && y < y_len - 1 => Ok(vec![
            (x - 1, y),
            (x - 1, y - 1),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
        ]),
        (x, y) if x > 0 && x < x_len - 1 && y == 0 => Ok(vec![
            (x - 1, y),
            (x - 1, y + 1),
            (x + 1, y),
            (x + 1, y + 1),
            (x, y + 1),
        ]),
        (x, y) if x > 0 && x < x_len - 1 && y == y_len - 1 => Ok(vec![
            (x - 1, y),
            (x - 1, y - 1),
            (x + 1, y),
            (x + 1, y - 1),
            (x, y - 1),
        ]),
        (x, y) if x == 0 && y == 0 => Ok(vec![(x + 1, y), (x + 1, y + 1), (x, y + 1)]),
        (x, y) if x == x_len - 1 && y == y_len - 1 => {
            Ok(vec![(x - 1, y), (x - 1, y - 1), (x, y - 1)])
        }
        (x, y) if x == x_len - 1 && y == 0 => Ok(vec![(x - 1, y), (x - 1, y + 1), (x, y + 1)]),
        (x, y) if x == 0 && y == y_len - 1 => Ok(vec![(x + 1, y), (x + 1, y - 1), (x, y - 1)]),
        (x, y) => Err(format!("Unhandled case. x: {}, y: {}", x, y)),
    }
}
