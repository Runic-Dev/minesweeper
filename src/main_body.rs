use crate::cell::Cell;
use crate::cell_data::{setup_game, CellType};
use leptos::*;

pub fn MainBody() -> impl IntoView {
    let game_state = create_rw_signal(setup_game(10, 20, 20));

    let update_cell = move |(row, col): (usize, usize)| {
        game_state.update(|state| {
            let cell = &mut state[row][col];
            if cell.cell_type == CellType::Bomb {
                print!("Game is lost!");
            }
            cell.open = true;
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
