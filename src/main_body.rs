use crate::cell::Cell;
use crate::cell_data::{
    setup_game,
    CellType::{Bomb, Number},
};
use leptos::*;

pub fn MainBody() -> impl IntoView {
    let game_state = create_rw_signal(setup_game(10, 20, 20));

    let update_cell = move |(row, col): (usize, usize)| {
        game_state.update(|state| {
            let state_clone = state.clone();
            let cell = &mut state[row][col];
            if cell.cell_type == Bomb {
                print!("Game is lost!");
            }

            match (row, col) {
                (r, c) if r > 0 && r < state_clone.len() && c > 0 && c < state_clone[row].len() => {
                    for coords in &[
                        (row - 1, col),
                        (row - 1, col - 1),
                        (row - 1, col + 1),
                        (row + 1, col),
                        (row + 1, col - 1),
                        (row + 1, col + 1),
                        (row, col - 1),
                        (row, col + 1),
                    ] {
                        if state_clone[coords.0][coords.1].cell_type == Bomb {
                            cell.number += 1;
                        }
                    }
                }
                (_, _) => {}
            }

            cell.open = true;

            if cell.cell_type == Number && cell.number > 0 {
                cell.content = format!("{}", cell.number);
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
