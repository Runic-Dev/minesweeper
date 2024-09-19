mod app;
mod cell;
mod cell_state;
mod click_mode;
mod game_state;
mod header;
mod main_body;

use crate::app::App;
use leptos::{mount_to_body, view};

fn main() {
    mount_to_body(|| view! { <App /> })
}
