use leptos::logging::log;
use leptos::*;
use wasm_bindgen::JsCast;

use crate::tile_state::{TileState, TileType};

#[component]
pub fn Tile(
    row: usize,
    col: usize,
    #[prop(into)] cell_data: Memo<TileState>,
    on_click: impl Fn((usize, usize)) + 'static,
    on_rmb_click: impl Fn((usize, usize), (i32, i32)) + 'static,
    // on_touchstart: impl Fn((usize, usize), (i32, i32)) + 'static,
) -> impl IntoView {
    let lmb_click_handler = move |_| {
        if !cell_data.get().is_open {
            on_click((row, col));
        }
    };

    // let touchstart_handler = move |ev: leptos::ev::TouchEvent| {
    //     if let Some(touch_event) = ev.dyn_ref::<web_sys::TouchEvent>() {
    //         if let Some(touch) = touch_event.touches().get(0) {
    //             log!("Touches: {} {}", touch.client_x(), touch.client_y());
    //             on_touchstart((row, col), (touch.client_x(), touch.client_y()));
    //         }
    //     }
    // };

    let cell_classes = move || {
        let open_bg_color = "bg-slate-800";
        [
            "w-10",
            "h-10",
            "my-1",
            "rounded",
            "flex",
            "justify-center",
            "items-center",
            match (cell_data.get().cell_type, cell_data.get().is_open) {
                (TileType::Number { local_bombs: 1 }, true) => "{open_bg_color} text-cyan-500",
                (TileType::Number { local_bombs: 2 }, true) => "{open_bg_color} text-lime-500",
                (TileType::Number { local_bombs: 3 }, true) => "{open_bg_color} text-fuchsia-500",
                (TileType::Number { local_bombs: 4 }, true) => "{open_bg_color} text-pink-500",
                (TileType::Number { local_bombs: 5 }, true) => "{open_bg_color} text-rose-500",
                (TileType::Number { local_bombs: _ }, true) => open_bg_color,
                (TileType::Bomb, true) => "bg-red-200",
                (_, false) => "bg-slate-200 text-slate-800",
            },
        ]
        .join(" ")
    };

    let get_content = move || match cell_data.get().cell_type {
        TileType::Number { local_bombs } if local_bombs > 0 && cell_data.get().is_open => {
            local_bombs.to_string()
        }
        _ => {
            if cell_data.get().flagged {
                return String::from("F");
            }
            String::new()
        }
    };

    let rmb_click_handler = move |mouse_event: leptos::ev::MouseEvent| {
        mouse_event.prevent_default();
        on_rmb_click((row, col), (mouse_event.client_x(), mouse_event.client_y()));
    };

    view! {
        <div class=cell_classes on:click=lmb_click_handler on:contextmenu=rmb_click_handler>{move || get_content() }</div>
    }
}
