use leptos::{component, view, IntoView, ReadSignal, SignalGet};

#[component]
pub fn TouchMenu(position: ReadSignal<(i32, i32)>) -> impl IntoView {
    let left_pos = move || format!("{}px", position.get().0);
    let top_pos = move || format!("{}px", position.get().1);
    view! {
        <div class="w-1/3 flex justify-between bg-indigo-700 py-6 px-2 rounded absolute"
            style:left=left_pos style:top=top_pos
            >
            <div>Flag</div>
            <div>Dig</div>
        </div>
    }
}
