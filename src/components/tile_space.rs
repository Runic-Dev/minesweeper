use leptos::*;

use crate::{
    app::GameStateSetter,
    components::tiles::tile_content::{
        bomb_content::BombImg, number_content::NumberContent, undug_content::UndugTileContent,
    },
    tile_state::{TileState, TileType},
};

#[component]
pub fn TileSpace(
    row: usize,
    col: usize,
    #[prop(into)] cell_data: Memo<TileState>,
    on_click: impl Fn((usize, usize)) + 'static,
    on_rmb_click: impl Fn((usize, usize), (i32, i32)) + 'static,
) -> impl IntoView {
    let game_state = use_context::<GameStateSetter>().unwrap().0;
    let lmb_click_handler = move |_| {
        if !cell_data.get().is_dug {
            on_click((row, col));
        }
    };

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
            match (cell_data.get().cell_type, cell_data.get().is_dug) {
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
        TileType::Number {
            local_bombs: local_mines,
        } if cell_data.get().is_dug => {
            view! {
                <NumberContent number=local_mines />
            }
        }
        TileType::Bomb if cell_data.get().is_dug => {
            view! {
                <BombImg />
            }
        }
        _ => {
            view! {
                <UndugTileContent is_flagged=cell_data.get().is_flagged />
            }
        }
    };

    let rmb_click_handler = move |mouse_event: leptos::ev::MouseEvent| {
        mouse_event.prevent_default();
        if !cell_data.get().is_dug && !game_state.get().game_over {
            on_rmb_click((row, col), (mouse_event.client_x(), mouse_event.client_y()));
        }
    };

    view! {
        <div class=cell_classes on:click=lmb_click_handler on:contextmenu=rmb_click_handler>{move || get_content() }</div>
    }
}
