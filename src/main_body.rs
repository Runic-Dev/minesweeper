use crate::cell::Cell;
use crate::cell_state::{get_neighbours, setup_game, CellType};
use leptos::logging::log;
use leptos::*;

pub fn MainBody() -> impl IntoView {
    let game_state = create_rw_signal(setup_game(10, 20, 20));

    let update_cell = move |(row, col): (usize, usize)| {
        game_state.update(|state| {
            let this_cell = &mut state[row][col];
            this_cell.is_open = true;
            if let CellType::Number { local_bombs: 0 } = this_cell.cell_type {
                match get_neighbours(row, col, state.len(), state[row].len()) {
                    Ok(neighbours) => {
                        neighbours.into_iter().for_each(|(x, y)| {
                            state[x][y].is_open = true;
                        });
                    }
                    Err(message) => log!("{}", message),
                }
            }
        });
    };

    view! {
        <div class="flex justify-center">
            <div class="game-container grid grid-cols-10 gap-2 border-double border-4 border-slate-200 rounded">
                <For
                    each=move || 0..game_state.get().len()
                    key=|&row| row
                    children=move |row| {
                        view! {
                            <div class="row">
                                <For
                                    each=move || 0..game_state.get()[row].len()
                                    key=|&col| col
                                    children=move |col| {
                                        let cell_data = create_memo(move |_| {
                                            game_state.with(|state| state[row][col].clone())
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
    }
}
