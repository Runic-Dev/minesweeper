use leptos::logging::log;
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
        log!("Row: {}, Col: {}", row, col);
        on_click((row, col));
    };

    let cell_class = move || {
        let cell = cell_data.get();
        let mut classes = vec![
            "w-10",
            "h-10",
            "my-1",
            "rounded",
            "flex",
            "justify-center",
            "items-center",
        ];

        if cell.is_open {
            classes.push("bg-slate-800");
        } else {
            classes.push("bg-slate-200");
        }

        if let CellType::Number { local_bombs } = cell.cell_type {
            match local_bombs {
                1 => classes.push("text-cyan-500"),
                2 => classes.push("text-lime-500"),
                3 => classes.push("text-fuchsia-500"),
                4 => classes.push("text-pink-500"),
                5 => classes.push("text-rose-500"),
                _ => {}
            };
        }

        classes.join(" ")
    };

    let get_content = move || match cell_data.get().cell_type {
        CellType::Number { local_bombs } if local_bombs > 0 && cell_data.get().is_open => {
            format!("{}", local_bombs)
        }
        _ => String::new(),
    };

    view! {
        <div class=cell_class on:click=handle_click>{move || get_content() }</div>
    }
}
