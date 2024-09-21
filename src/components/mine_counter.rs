use leptos::{component, view, IntoView};

#[component]
pub fn MineCounter(mines: usize, is_flagged: bool) -> impl IntoView {
    if is_flagged {
        view! {
            <p>F</p>
        }
    } else if mines > 0 {
        view! {
            <p>{mines}</p>
        }
    } else {
        view! {
            <p></p>
        }
    }
}
