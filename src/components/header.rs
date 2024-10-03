use leptos::{component, view, IntoView};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="text-xl font-bold p-2 bg-slate-600">Minesweeper</header>
    }
}
