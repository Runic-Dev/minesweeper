use crate::app::GameStateSetter;
use crate::components::context_menu::ContextMenu;
use crate::components::main_body::{check_for_surrounding_blanks, has_won};
use crate::components::tile_space::TileSpace;
use crate::game_state::PlayState::{InProgress, Loss, Win};
use crate::tile_state::TileType;
use leptos::logging::log;
use leptos::*;
use leptos_use::use_document;

#[component]
pub fn MainBody() -> impl IntoView {
    let game_state = use_context::<GameStateSetter>().unwrap().0;

    let refresh = move |_| {
        let document = use_document();
        document.location().map_or_else(
            || log!("Error getting location"),
            |location| {
                let _ = location.reload();
            },
        );
    };

    let dig_tile = move |(row, col): (usize, usize)| {
        if let InProgress { mines_left: _ } = game_state.get().play_state {
            game_state.update(|state| {
                state.grid[row][col].is_dug = true;
                match state.grid[row][col].cell_type {
                    TileType::Number { local_mines: _ } if state.grid[row][col].is_flagged => {
                        state.grid[row][col].is_flagged = false;
                    }
                    TileType::Number { local_mines: 0 } => {
                        check_for_surrounding_blanks(row, col, &mut state.grid);
                        if has_won(&state.grid) {
                            state.play_state = Win;
                        };
                    }
                    TileType::Number { local_mines: _ } => {
                        if has_won(&state.grid) {
                            state.play_state = Win;
                        };
                    }
                    TileType::Bomb => {
                        state
                            .grid
                            .iter_mut()
                            .flatten()
                            .filter(|tile_state| tile_state.cell_type == TileType::Bomb)
                            .for_each(|mine_tile| mine_tile.is_dug = true);
                        state.play_state = Loss;
                    }
                }
            });
        }
    };

    let flag_tile = move |(row, col): (usize, usize)| {
        game_state
            .update(|state| state.grid[row][col].is_flagged = !state.grid[row][col].is_flagged);
        if has_won(&game_state.get().grid) {
            game_state.update(|state| state.play_state = Win);
        }
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
        <div class="flex justify-center items-center">
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
            <Show when=move || matches!(game_state.get().play_state, Win)>
                <div class="absolute p-4 flex flex-col justify-center items-center bg-green-800 rounded min-h-fit w-1/3">
                    <p class="p-2">You Won!</p>
                    <button class="bg-green-600 p-2 rounded" on:click=refresh>Play again</button>
                </div>
            </Show>
            <Show when=move || matches!(game_state.get().play_state, Loss)>
                <div class="absolute p-4 flex flex-col justify-center items-center bg-red-800 rounded min-h-fit w-1/3">
                    <p class="p-2">You Lost!</p>
                    <button class="bg-red-600 p-2 rounded" on:click=refresh>Play again</button>
                </div>
            </Show>
        </div>
    }
}
