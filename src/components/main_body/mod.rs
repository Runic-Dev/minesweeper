use leptos::logging::log;

use crate::{
    game_state::get_neighbours,
    tile_state::{TileState, TileType},
};

pub mod main_body_component;

fn has_won(game_grid: &[Vec<TileState>]) -> bool {
    game_grid
        .iter()
        .flatten()
        .map(
            |tile| match (tile.cell_type.clone(), tile.is_flagged, tile.is_dug) {
                (TileType::Number { local_mines: _ }, true, false) => false,
                (TileType::Number { local_mines: _ }, _, true) => true,
                (TileType::Number { local_mines: _ }, false, false) => false,
                (TileType::Bomb, true, _) => true,
                (TileType::Bomb, false, _) => false,
            },
        )
        .all(|tile| tile)
}

fn check_for_surrounding_blanks(row: usize, col: usize, state: &mut [Vec<TileState>]) {
    match get_neighbours(row, col, state.len(), state[row].len()) {
        Ok(neighbours) => {
            let closed_neighbours = neighbours
                .into_iter()
                .filter(|(x, y)| {
                    !state[*x][*y].is_dug
                        && matches!(state[*x][*y].cell_type, TileType::Number { local_mines: _ })
                })
                .collect::<Vec<(usize, usize)>>();

            closed_neighbours.into_iter().for_each(|(x, y)| {
                let this_cell = &mut state[x][y];
                this_cell.is_dug = true;
                if let TileType::Number { local_mines } = this_cell.cell_type {
                    if local_mines == 0 {
                        check_for_surrounding_blanks(x, y, state);
                    }
                }
            });
        }
        Err(message) => log!("Got an error: {}", message),
    }
}
