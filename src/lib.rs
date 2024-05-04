#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use components::login::Login;
use components::chat::Chat;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/chat")]
    Chat,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(selected_route: &Route) -> Html {
    match selected_route {
        Route::Login => html! {<Login />},
        Route::Chat => html! {<Chat/>},
        Route::NotFound => html! {<h1>{"404 baby"}</h1>},
    }
}

#[function_component(Main)]
fn main() -> Html {
    html! {
        <BrowserRouter>
            <div class="flex w-screen h-screen">
                <Switch<Route> render={Switch::render(switch)}/>
            </div>
        </BrowserRouter>
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
    Ok(())
}