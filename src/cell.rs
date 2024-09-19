use leptos::*;

use crate::cell_state::{CellState, CellType};

#[component]
pub fn Cell(
    row: usize,
    col: usize,
    #[prop(into)] cell_data: Memo<CellState>,
    on_click: impl Fn((usize, usize)) + 'static,
) -> impl IntoView {
    let handle_click = move |_| {
        on_click((row, col));
    };

    let cell_classes = move || {
        let open_bg_color = "bg-slate-800";
        [
            String::from("w-10"),
            String::from("h-10"),
            String::from("my-1"),
            String::from("rounded"),
            String::from("flex"),
            String::from("justify-center"),
            String::from("items-center"),
            match (cell_data.get().cell_type, cell_data.get().is_open) {
                (CellType::Number { local_bombs: 1 }, true) => {
                    format!("{} text-cyan-500", open_bg_color)
                }
                (CellType::Number { local_bombs: 2 }, true) => {
                    format!("{} text-lime-500", open_bg_color)
                }
                (CellType::Number { local_bombs: 3 }, true) => {
                    format!("{} text-fuchsia-500", open_bg_color)
                }
                (CellType::Number { local_bombs: 4 }, true) => {
                    format!("{} text-pink-500", open_bg_color)
                }
                (CellType::Number { local_bombs: 5 }, true) => {
                    format!("{} text-rose-500", open_bg_color)
                }
                (CellType::Number { local_bombs: _ }, true) => open_bg_color.to_string(),
                (CellType::Bomb, true) => "bg-red-800".to_string(),
                (_, false) => "bg-slate-200".to_string(),
            },
        ]
        .join(" ")
    };

    let get_content = move || match cell_data.get().cell_type {
        CellType::Number { local_bombs } if local_bombs > 0 && cell_data.get().is_open => {
            local_bombs.to_string()
        }
        _ => String::new(),
    };

    view! {
        <div class=cell_classes on:click=handle_click>{move || get_content() }</div>
    }
}
