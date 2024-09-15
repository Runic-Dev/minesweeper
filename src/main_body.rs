use crate::cell::Cell;
use crate::cell_state::{
    setup_game,
    CellType::{Bomb, Number},
};
use leptos::logging;
use leptos::*;
use logging::log;

pub fn MainBody() -> impl IntoView {
    let game_state = create_rw_signal(setup_game(10, 20, 20));

    let update_cell = move |(row, col): (usize, usize)| {
        game_state.update(|state| {
            let state_clone = state.clone();
            let cell = &mut state[row][col];

            match cell.cell_type {
                Number { mut local_bombs } => {
                    let coords_result = match (row, col) {
                        (r, c)
                            if r > 0
                                && r < state_clone.len() - 1
                                && c > 0
                                && c < state_clone[row].len() - 1 =>
                        {
                            log!(
                                "state_clone len: {}, state_clone row len: {}",
                                state_clone.len(),
                                state_clone[row].len()
                            );
                            log!("Non edge case");
                            Ok(vec![
                                (row - 1, col),
                                (row - 1, col - 1),
                                (row - 1, col + 1),
                                (row + 1, col),
                                (row + 1, col - 1),
                                (row + 1, col + 1),
                                (row, col - 1),
                                (row, col + 1),
                            ])
                        }
                        (r, c) if r > 0 && r < state_clone.len() - 1 && c == 0 => {
                            log!(
                                "state_clone len: {}, state_clone row len: {}",
                                state_clone.len(),
                                state_clone[row].len()
                            );
                            log!("Column is 0");
                            Ok(vec![
                                (row - 1, col),
                                (row - 1, col + 1),
                                (row + 1, col),
                                (row + 1, col + 1),
                                (row, col + 1),
                            ])
                        }
                        (r, c)
                            if r > 0
                                && r < state_clone.len() - 1
                                && c == state_clone[row].len() - 1 =>
                        {
                            log!(
                                "state_clone len: {}, state_clone row len: {}",
                                state_clone.len(),
                                state_clone[row].len()
                            );
                            log!("Column is on the end");
                            Ok(vec![
                                (row - 1, col),
                                (row - 1, col - 1),
                                (row + 1, col),
                                (row + 1, col - 1),
                                (row, col - 1),
                            ])
                        }
                        (r, c) if r == 0 && c > 0 && c < state_clone[row].len() - 1 => {
                            log!(
                                "state_clone len: {}, state_clone row len: {}",
                                state_clone.len(),
                                state_clone[row].len()
                            );
                            log!("Row is 0");
                            Ok(vec![
                                (row + 1, col),
                                (row + 1, col - 1),
                                (row + 1, col + 1),
                                (row, col - 1),
                                (row, col + 1),
                            ])
                        }
                        (r, c)
                            if r == state_clone.len() - 1
                                && c > 0
                                && c < state_clone[row].len() - 1 =>
                        {
                            log!(
                                "state_clone len: {}, state_clone row len: {}",
                                state_clone.len(),
                                state_clone[row].len()
                            );
                            log!("Row is on the end");
                            Ok(vec![
                                (row - 1, col),
                                (row - 1, col - 1),
                                (row - 1, col + 1),
                                (row, col - 1),
                                (row, col + 1),
                            ])
                        }
                        (r, c) => Err(format!("Unhandled case. Row: {}, Col: {}", r, c)),
                    };

                    match coords_result {
                        Ok(coords) => {
                            for (x, y) in coords {
                                if state_clone[x][y].cell_type == Bomb {
                                    local_bombs += 1;
                                }
                            }
                        }
                        Err(message) => log!("{}", message),
                    }

                    cell.content = format!("{}", local_bombs);
                }
                Bomb => log!("Game is lost!"),
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
