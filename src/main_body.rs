use crate::cell::Cell;
use crate::cell_state::{CellState, CellType};
use crate::click_mode::ClickMode;
use crate::context_menu::ContextMenu;
use crate::game_state::{get_neighbours, GameState};
use leptos::logging::log;
use leptos::*;

#[component]
pub fn MainBody(game_state: RwSignal<GameState>) -> impl IntoView {
    let update_cell = move |(row, col): (usize, usize)| {
        game_state.update(|state| match state.click_mode {
            ClickMode::Dig => {
                state.grid[row][col].is_open = true;
                if let CellType::Number { local_bombs } = state.grid[row][col].cell_type {
                    if local_bombs == 0 {
                        check_for_surrounding_blanks(row, col, &mut state.grid);
                    }
                }
            }
            ClickMode::Flag => {
                state.grid[row][col].flagged = !state.grid[row][col].flagged;
            }
        });
    };

    let toggle_click_mode = move |_| {
        game_state.update(|state| {
            if let ClickMode::Dig = state.click_mode {
                state.click_mode = ClickMode::Flag;
            } else {
                state.click_mode = ClickMode::Dig;
            }
        });
    };

    let mut hidden = false;

    let right_click_handler = move |mouse_event: leptos::ev::MouseEvent| {
        mouse_event.prevent_default();
        hidden = !hidden;
    };

    let get_content = move || match game_state.get().click_mode {
        ClickMode::Dig => "Dig",
        ClickMode::Flag => "Flag",
    };

    view! {
        <div>
        <div class="flex justify-center">
            <button class="bg-slate-200 text-slate-800 mb-1" on:click=toggle_click_mode>{ move || get_content() }</button>
        </div>
        <div class="flex justify-center">
            <div on:contextmenu=right_click_handler class="game-container grid grid-cols-10 gap-x-1 gap-y-1 m-0 px-1 border-double border-4 border-slate-200 rounded">
                <ContextMenu hidden={hidden} />
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
                                            <Cell row=row col=col cell_data=cell_data on_click=update_cell />
                                        }
                                    }
                                />
                            </div>
                        }
                    }
                />
            </div>
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
