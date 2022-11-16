mod layout;
mod pages;

use crate::layout::navbar::NavBar;
use crate::pages::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    HomePage,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! { <Home /> },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <NavBar />
            <BrowserRouter>
                <Switch<Route> pathname="/" render={switch} />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
