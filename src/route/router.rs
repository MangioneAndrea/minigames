use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::home::Home;
use crate::pages::games::games::Games;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/games")]
    GamesPage,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! { <Home /> },
        Route::GamesPage => html! { <Games/> },
        Route::NotFound => html! { <h1>{"404"}</h1> },
    }
}
