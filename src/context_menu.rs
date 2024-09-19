use leptos::{component, view, IntoView};

#[component]
pub fn ContextMenu(hidden: bool) -> impl IntoView {
    let context_menu_classes = move || {
        if !hidden {
            return "bg-slate-200 text-slate-800 hidden";
        }
        "bg-slate-200 text-slate-800"
    };
    view! {
        <div class=context_menu_classes>Hey there!</div>
    }
}
