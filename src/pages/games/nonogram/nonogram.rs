use crate::pages::games::nonogram::grid::Grid2;
use yew::prelude::*;

#[function_component(Nonogram)]
pub fn nonogram() -> Html {
    html! {
        <div>
            <h1>{"Nonogram"}</h1>
            <p>{"Nonogram is a game where you have to fill in a grid with black or whites. Rows and columns tell you what how many blocks should be conseguetially painted black"}</p>
            <Grid2 cols={4} rows={4} rows_rules={vec![
                vec![],
                vec![3],
                vec![],
                vec![1, 1],
            ]} cols_rules={vec![
                vec![],
                vec![],
                vec![],
                vec![1,1],
            ]}/>
        </div>
    }
}
