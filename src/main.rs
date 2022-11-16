mod pages;
mod route;

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
    yew::Renderer::<App>::new().render();
}
