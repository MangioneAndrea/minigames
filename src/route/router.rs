use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::home::Home;
use crate::pages::games::sukodu::sukodu::Sukodu;
use crate::pages::games::nonnogrampa::nonogram::NonnoGrampa;
use crate::pages::games::snakent::snakent::Snakent;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/sudoku")]
    SukoduPage,
    #[at("/nonogram")]
    NonogramPage,
    #[at("/snake")]
    SnakePage,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! { <Home /> },
        Route::SukoduPage => html! { <Sukodu/> },
        Route::NonogramPage => html! { <NonnoGrampa/> },
        Route::SnakePage => html! { <Snakent/> },
        Route::NotFound => html! { <h1>{"404"}</h1> },
    }
}
