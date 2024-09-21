use leptos::logging::log;

use crate::{
    game_state::get_neighbours,
    tile_state::{TileState, TileType},
};

pub mod main_body;

fn check_for_surrounding_blanks(row: usize, col: usize, state: &mut [Vec<TileState>]) {
    match get_neighbours(row, col, state.len(), state[row].len()) {
        Ok(neighbours) => {
            let closed_neighbours = neighbours
                .into_iter()
                .filter(|(x, y)| {
                    !state[*x][*y].is_dug
                        && matches!(state[*x][*y].cell_type, TileType::Number { local_bombs: _ })
                })
                .collect::<Vec<(usize, usize)>>();

            closed_neighbours.into_iter().for_each(|(x, y)| {
                let this_cell = &mut state[x][y];
                this_cell.is_dug = true;
                if let TileType::Number { local_bombs } = this_cell.cell_type {
                    if local_bombs == 0 {
                        check_for_surrounding_blanks(x, y, state);
                    }
                }
            });
        }
        Err(message) => log!("Got an error: {}", message),
    }
}
