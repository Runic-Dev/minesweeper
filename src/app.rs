use crate::header::Header;
use crate::main_body::MainBody;
use leptos::component;
use leptos::view;
use leptos::IntoView;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="container mx-auto flex flex-col justify-center">
            <Header />
            <MainBody />
        </div>
    }
}
