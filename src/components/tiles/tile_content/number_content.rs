use leptos::{component, view, IntoView};

use super::get_generic_classes;

#[component]
pub fn NumberContent(number: usize) -> impl IntoView {
    let mut classes = get_generic_classes();
    let classes_for_number = match number {
        1 => "bg-slate-800 text-cyan-500",
        2 => "bg-slate-800 text-lime-500",
        3 => "bg-slate-800 text-fuchsia-500",
        4 => "bg-slate-800 text-pink-500",
        5 => "bg-slate-800 text-rose-500",
        _ => "bg-slate-800",
    };
    classes.push(classes_for_number);
    let classes = classes.join(" ");

    if number > 0 {
        view! {
            <span class=classes>{number}</span>
        }
    } else {
        view! {
            <span class=classes></span>
        }
    }
}
