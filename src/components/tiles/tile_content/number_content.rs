use leptos::{component, view, IntoView};

#[component]
pub fn NumberContent(number: usize) -> impl IntoView {
    if number > 0 {
        view! {
            <span>{number}</span>
        }
    } else {
        view! {
            <span></span>
        }
    }
}
