use super::grid::Grid;
use crate::rule;
use yew::prelude::*;

use super::core::Rule;

struct NonogramType {
    cols: usize,
    rows: usize,
    cols_rules: Vec<Rule>,
    rows_rules: Vec<Rule>,
}

#[function_component(Nonogram)]
pub fn nonogram() -> Html {
    let nonogram_example: NonogramType = NonogramType {
        cols: 15,
        rows: 15,
        cols_rules: vec![
            rule![9],
            rule![9, 5],
            rule![5, 2, 6],
            rule![3, 2, 3],
            rule![2, 1, 2, 2],
            rule![3, 1, 3, 1],
            rule![5, 3, 1],
            rule![1, 2, 1],
            rule![2, 2, 1, 1],
            rule![2, 2, 1, 1, 1],
            rule![2, 1, 2, 1, 1],
            rule![2, 1, 2, 1],
            rule![4, 4, 1],
            rule![11, 1],
            rule![7, 3],
        ],
        rows_rules: vec![
            rule![5],
            rule![6],
            rule![4, 1, 4],
            rule![3, 2, 6],
            rule![3, 1, 1, 2, 3],
            rule![2, 1, 1, 1, 3],
            rule![3, 2, 1, 2],
            rule![3, 1, 1, 2],
            rule![2, 3, 5],
            rule![7, 1, 3],
            rule![2, 2, 2, 2],
            rule![2, 2, 3],
            rule![3, 5, 2],
            rule![4, 1],
            rule![14],
        ],
    };

    html! {
        <div>
            <h1>{"Nonogram"}</h1>
            <p>{"Nonogram is a game where you have to fill in a grid with black or whites. Rows and columns tell you what how many blocks should be conseguetially painted black"}</p>
            <p>{"Of course I'm an evil person, so the grid randomly rotates"}</p>
            <Grid
                cols={nonogram_example.cols}
                rows={nonogram_example.rows}
                rows_rules={nonogram_example.rows_rules}
                cols_rules={nonogram_example.cols_rules}
            />
        </div>
    }
}
