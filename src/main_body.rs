use crate::cell::Cell;
use crate::cell_state::{CellState, CellType};
use crate::context_menu::ContextMenu;
use crate::game_state::{get_neighbours, GameState};
use leptos::logging::log;
use leptos::*;

#[component]
pub fn MainBody(game_state: RwSignal<GameState>) -> impl IntoView {
    let dig_tile = move |(row, col): (usize, usize)| {
        game_state.update(|state| {
            state.grid[row][col].is_open = true;
            if let CellType::Number { local_bombs: 0 } = state.grid[row][col].cell_type {
                check_for_surrounding_blanks(row, col, &mut state.grid);
            }
        });
    };

    let flag_tile = move |(row, col): (usize, usize)| {
        game_state.update(|state| state.grid[row][col].flagged = !state.grid[row][col].flagged);
    };

    let (hidden, set_hidden) = create_signal(true);

    let (position, set_position) = create_signal((0, 0));

    let (ctx_menu_cell, set_ctx_menu_cell) = create_signal(None);

    let on_ctx_menu_select = move |(row, col), (x, y): (i32, i32)| {
        set_position((x, y));
        set_hidden(false);
        set_ctx_menu_cell.update(|value| *value = Some((row, col)));
    };
    let on_ctx_menu_dig = move || {
        if let Some(tile_coords) = ctx_menu_cell.get() {
            dig_tile(tile_coords);
        }
        set_hidden(true);
    };
    let on_ctx_menu_flag = move || {
        if let Some(tile_coords) = ctx_menu_cell.get() {
            flag_tile(tile_coords);
        }
        set_hidden(true);
    };
    let on_ctx_menu_cancel = move || {
        set_ctx_menu_cell.update(|value| *value = None);
        set_hidden(true);
    };

    view! {
        <div class="flex justify-center">
            <div class="game-container grid grid-cols-10 gap-x-1 gap-y-1 m-0 px-1 border-double border-4 border-slate-200 rounded">
                <Show when=move || !hidden.get()>
                    <ContextMenu position=position on_dig=on_ctx_menu_dig on_flag=on_ctx_menu_flag on_cancel=on_ctx_menu_cancel />
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
                                            <Cell row=row col=col cell_data=cell_data on_click=dig_tile on_rmb_click=on_ctx_menu_select />
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

fn check_for_surrounding_blanks(row: usize, col: usize, state: &mut [Vec<CellState>]) {
    match get_neighbours(row, col, state.len(), state[row].len()) {
        Ok(neighbours) => {
            let closed_neighbours = neighbours
                .into_iter()
                .filter(|(x, y)| {
                    !state[*x][*y].is_open
                        && matches!(state[*x][*y].cell_type, CellType::Number { local_bombs: _ })
                })
                .collect::<Vec<(usize, usize)>>();

            closed_neighbours.into_iter().for_each(|(x, y)| {
                let this_cell = &mut state[x][y];
                this_cell.is_open = true;
                if let CellType::Number { local_bombs } = this_cell.cell_type {
                    if local_bombs == 0 {
                        check_for_surrounding_blanks(x, y, state);
                    }
                }
            });
        }
        Err(message) => log!("Got an error: {}", message),
    }
}
