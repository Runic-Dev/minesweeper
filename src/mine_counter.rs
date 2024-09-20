use leptos::{component, view, IntoView};

#[component]
pub fn MineCounter(mines: usize) -> impl IntoView {
    view! {
        <p>{mines}</p>
    }
}
