mod app;
mod components;
mod game_state;
mod tile_state;

use crate::app::App;
use leptos::{mount_to_body, view};

fn main() {
    mount_to_body(|| view! { <App /> })
}
