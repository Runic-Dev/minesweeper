use leptos::{component, view, IntoView, ReadSignal, SignalGet};

#[component]
pub fn ContextMenu(position: ReadSignal<(i32, i32)>) -> impl IntoView {
    let left_pos = move || format!("{}px", position.get().0);
    let top_pos = move || format!("{}px", position.get().1);
    view! {
        <div class="bg-slate-200 text-slate-800 absolute" style:left=left_pos style:top=top_pos>Hey there!</div>
    }
}
