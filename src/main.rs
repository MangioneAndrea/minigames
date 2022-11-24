mod pages;
mod route;
mod engine;

use crate::route::navbar::NavBar;
use crate::route::router::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <BrowserRouter>
                <NavBar />
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
