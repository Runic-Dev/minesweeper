use leptos::{component, view, IntoView};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="flex justify-center text-xl font-bold">Minesweeper</header>
    }
}
