use leptos::*;

use crate::cell_data::{CellData, CellType};

#[component]
pub fn Cell(
    row: usize,
    col: usize,
    #[prop(into)] cell_data: Memo<CellData>,
    on_click: impl Fn((usize, usize)) + 'static,
) -> impl IntoView {
    let handle_click = move |_| {
        on_click((row, col));
    };

    let cell_class = move || {
        let cell = cell_data.get();
        let mut classes = vec![
            "w-10",
            "h-10",
            "m-1",
            "rounded",
            "flex",
            "justify-center",
            "items-center",
        ];
        match cell.open {
            true if cell.cell_type == CellType::Bomb => classes.push("bg-red-200"),
            true => classes.push("bg-slate-800"),
            false => classes.push("bg-slate-200"),
        };
        classes.join(" ")
    };

    view! {
        <div class=cell_class on:click=handle_click>{move || cell_data.get().content }</div>
    }
}
