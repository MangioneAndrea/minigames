mod layout;
mod pages;

use crate::layout::navbar::NavBar;
use crate::pages::games::snakent::snakent::Snakent;
use crate::pages::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    HomePage,
    #[at("/snakent")]
    SnakentPage,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! { <Home /> },
        Route::SnakentPage => html! { <Snakent/> },
        Route::NotFound => html! { <h1>{"404"}</h1> },
    }
}

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
