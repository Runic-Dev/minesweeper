use crate::game_state::GameState;
use crate::header::Header;
use crate::main_body::MainBody;
use leptos::component;
use leptos::create_rw_signal;
use leptos::view;
use leptos::IntoView;

#[component]
pub fn App() -> impl IntoView {
    let game_state = create_rw_signal(GameState::default());
    view! {
        <div class="container mx-auto flex flex-col justify-center">
            <Header />
            <MainBody game_state />
        </div>
    }
}
