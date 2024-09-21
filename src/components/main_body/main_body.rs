use crate::app::GameStateSetter;
use crate::components::context_menu::ContextMenu;
use crate::components::tile_space::TileSpace;
use crate::game_state::get_neighbours;
use crate::tile_state::{TileState, TileType};
use leptos::logging::log;
use leptos::*;

#[component]
pub fn MainBody() -> impl IntoView {
    let game_state = use_context::<GameStateSetter>().unwrap().0;

    let dig_tile = move |(row, col): (usize, usize)| {
        if !game_state.get().game_over {
            game_state.update(|state| {
                state.grid[row][col].is_dug = true;
                if let TileType::Number { local_bombs: 0 } = state.grid[row][col].cell_type {
                    check_for_surrounding_blanks(row, col, &mut state.grid);
                }
                if let TileType::Bomb = state.grid[row][col].cell_type {
                    state
                        .grid
                        .iter_mut()
                        .flatten()
                        .filter(|tile_state| tile_state.cell_type == TileType::Bomb)
                        .for_each(|mine_tile| mine_tile.is_dug = true);
                    state.game_over = true;
                }
            });
        }
    };

    let flag_tile = move |(row, col): (usize, usize)| {
        game_state
            .update(|state| state.grid[row][col].is_flagged = !state.grid[row][col].is_flagged);
    };

    let (ctx_menu_hidden, set_ctx_menu_hidden) = create_signal(true);

    let (ctx_menu_pos, set_ctx_menu_pos) = create_signal((0, 0));

    let (ctx_menu_cell, set_ctx_menu_cell) = create_signal(None);

    let on_ctx_menu_select = move |(row, col), (x, y): (i32, i32)| {
        set_ctx_menu_pos((x, y));
        set_ctx_menu_hidden(false);
        set_ctx_menu_cell.update(|value| *value = Some((row, col)));
    };
    let on_ctx_menu_dig = move || {
        if let Some(tile_coords) = ctx_menu_cell.get() {
            dig_tile(tile_coords);
        }
        set_ctx_menu_hidden(true);
    };
    let on_ctx_menu_flag = move || {
        if let Some(tile_coords) = ctx_menu_cell.get() {
            flag_tile(tile_coords);
        }
        set_ctx_menu_hidden(true);
    };
    let on_ctx_menu_cancel = move || {
        set_ctx_menu_cell.update(|value| *value = None);
        set_ctx_menu_hidden(true);
    };

    view! {
        <div class="flex justify-center">
            <div class="game-container grid grid-cols-10 gap-x-1 gap-y-1 m-0 px-1 border-double border-4 border-slate-200 rounded">
                <Show when=move || !ctx_menu_hidden.get()>
                    <ContextMenu position=ctx_menu_pos on_dig=on_ctx_menu_dig on_flag=on_ctx_menu_flag on_cancel=on_ctx_menu_cancel />
                </Show>
                <For
                    each=move || 0..game_state.get().grid.len()
                    key=|&row| row
                    children=move |row| {
                        view! {
                            <div class="row">
                                <For
                                    each=move || 0..game_state.get().grid[row].len()
                                    key=|&col| col
                                    children=move |col| {
                                        let cell_data = create_memo(move |_| {
                                            game_state.with(|state| state.grid[row][col].clone())
                                        });
                                        view! {
                                            <TileSpace row=row col=col cell_data=cell_data on_click=dig_tile on_rmb_click=on_ctx_menu_select />
                                        }
                                    }
                                />
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

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
