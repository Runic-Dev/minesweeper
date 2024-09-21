use leptos::{component, view, IntoView};

use crate::components::tiles::tile_content::get_generic_styles;

#[component]
pub fn UndugTileContent(is_flagged: bool) -> impl IntoView {
    let mut classes = get_generic_styles();
    classes.extend(&["bg-slate-200", "text-slate-800"]);
    let styles = classes.join(" ");

    let content = if is_flagged { "F" } else { "" };

    view! {
        <div class=styles>{move || content}</div>
    }
}
