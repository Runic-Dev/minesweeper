use crate::components::header::Header;
use crate::components::main_body::main_body::MainBody;
use crate::game_state::GameState;
use leptos::component;
use leptos::create_rw_signal;
use leptos::provide_context;
use leptos::view;
use leptos::IntoView;
use leptos::RwSignal;

#[derive(Copy, Clone)]
pub struct GameStateSetter(pub RwSignal<GameState>);

#[component]
pub fn App() -> impl IntoView {
    let game_state = create_rw_signal(GameState::default());
    provide_context(GameStateSetter(game_state));
    view! {
        <div class="container mx-auto flex flex-col justify-center">
            <Header />
            <MainBody />
        </div>
    }
}
