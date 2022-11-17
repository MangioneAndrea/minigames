use yew::prelude::*;
use crate::pages::games::nonogram::grid::Grid;

#[function_component(Nonogram)]
pub fn nonogram() -> Html {
    html! {
        <div>
            <h1>{"Nonogram"}</h1>
            <p>{"Nonogram is a game where you have to fill in a grid with black or whites. Rows and columns tell you what how many blocks should be conseguetially painted black"}</p>
            <Grid/>
        </div>
    }
}
