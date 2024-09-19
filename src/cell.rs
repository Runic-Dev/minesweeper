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
            "w-10",
            "h-10",
            "my-1",
            "rounded",
            "flex",
            "justify-center",
            "items-center",
            match (cell_data.get().cell_type, cell_data.get().is_open) {
                (CellType::Number { local_bombs: 1 }, true) => "{open_bg_color} text-cyan-500",
                (CellType::Number { local_bombs: 2 }, true) => "{open_bg_color} text-lime-500",
                (CellType::Number { local_bombs: 3 }, true) => "{open_bg_color} text-fuchsia-500",
                (CellType::Number { local_bombs: 4 }, true) => "{open_bg_color} text-pink-500",
                (CellType::Number { local_bombs: 5 }, true) => "{open_bg_color} text-rose-500",
                (CellType::Number { local_bombs: _ }, true) => open_bg_color,
                (CellType::Bomb, true) => "bg-red-800",
                (_, false) => "bg-slate-200",
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
