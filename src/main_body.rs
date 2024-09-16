use crate::cell::Cell;
use crate::cell_state::{get_neighbours, setup_game, CellState, CellType};
use leptos::logging::log;
use leptos::*;

fn check_for_surrounding_blanks(row: usize, col: usize, state: &mut [Vec<CellState>]) {
    log!("Check for surrounding blanks fired");
    match get_neighbours(row, col, state.len(), state[row].len()) {
        Ok(neighbours) => {
            log!("Successfully got neighbours");
            let closed_neighbours = neighbours
                .into_iter()
                .filter(|(x, y)| {
                    !state[*x][*y].is_open
                        && matches!(state[*x][*y].cell_type, CellType::Number { local_bombs: _ })
                })
                .collect::<Vec<(usize, usize)>>();

            log!(
                "Corr, we got ourselves {} closed_neighbours!",
                closed_neighbours.len()
            );

            closed_neighbours.into_iter().for_each(|(x, y)| {
                let this_cell = &mut state[x][y];
                log!("Opening up neighbour..");
                this_cell.is_open = true;
                if let CellType::Number { local_bombs } = this_cell.cell_type {
                    if local_bombs == 0 {
                        log!("neighbour's local_bombs is 0, checking for surrounding blanks..");
                        check_for_surrounding_blanks(x, y, state);
                    } else {
                        log!(
                            "neighbour's local_bombs is {}, not checking further",
                            local_bombs
                        );
                    }
                }
            });
        }
        Err(message) => log!("Got an error: {}", message),
    }
}

pub fn MainBody() -> impl IntoView {
    let game_state = create_rw_signal(setup_game(10, 20, 20));

    let update_cell = move |(row, col): (usize, usize)| {
        game_state.update(|state| {
            state[row][col].is_open = true;
            if let CellType::Number { local_bombs } = state[row][col].cell_type {
                if local_bombs == 0 {
                    log!("Local bombs on clicked is 0, checking neighbours..");
                    check_for_surrounding_blanks(row, col, state);
                }
            }
        });
    };

    view! {
        <div class="flex justify-center">
            <div class="game-container grid grid-cols-10 gap-x-1 gap-y-1 m-0 px-1 border-double border-4 border-slate-200 rounded">
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
