use leptos::{component, view, IntoView, ReadSignal, SignalGet};

#[component]
pub fn ContextMenu(
    position: ReadSignal<(i32, i32)>,
    on_dig: impl Fn() + 'static,
    on_flag: impl Fn() + 'static,
    on_cancel: impl Fn() + 'static,
) -> impl IntoView {
    let left_pos = move || format!("{}px", position.get().0);
    let top_pos = move || format!("{}px", position.get().1);
    let dig_btn_handler = move |_| {
        on_dig();
    };
    let flag_btn_handler = move |_| {
        on_flag();
    };
    let cancel_btn_handler = move |_| {
        on_cancel();
    };
    view! {
        <div class="p-1 bg-slate-200 text-slate-200 absolute border-solid border-2 border-black rounded" style:left=left_pos style:top=top_pos>
            <ul>
                <li class="px-2 py-1 text-black hover:bg-slate-700 hover:text-white cursor-pointer" on:click=dig_btn_handler>Dig</li>
                <li class="px-2 py-1 text-black hover:bg-slate-700 hover:text-white cursor-pointer" on:click=flag_btn_handler>Flag</li>
                <li class="px-2 py-1 bg-red-700 text-white hover:bg-red-300 cursor-pointer rounded" on:click=cancel_btn_handler>Cancel</li>
            </ul>
        </div>
    }
}
