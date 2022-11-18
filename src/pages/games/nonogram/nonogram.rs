use crate::pages::games::nonogram::grid::Grid2;
use yew::prelude::*;

struct NonogramType{
    cols: usize,
    rows: usize,
    cols_rules: Vec<Vec<usize>>,
    rows_rules: Vec<Vec<usize>>,
}

#[function_component(Nonogram)]
pub fn nonogram() -> Html {
    let nonogram_example: NonogramType = NonogramType{
        cols: 15,
        rows: 15,
        cols_rules: vec![
            vec![9],
            vec![9,5],
            vec![5,2,6],
            vec![3,2,3],
            vec![2,1,2,2],
            vec![3,1,3,1],
            vec![5,3,1],
            vec![1,2,1],
            vec![2,2,1,1],
            vec![2,2,1,1,1],
            vec![2,1,2,1,1],
            vec![2,1,2,1],
            vec![4,4,1],
            vec![11,1],
            vec![7,3],
        ], 
        rows_rules: vec![
            vec![5],
            vec![6],
            vec![4,1,4],
            vec![3,2,6],
            vec![3,1,1,2,3],
            vec![2,1,1,1,3],
            vec![3,2,1,2],
            vec![3,1,1,2],
            vec![2,3,5],
            vec![7,1,3],
            vec![2,2,2,2],
            vec![2,2,3],
            vec![2,5,1],
            vec![4,1],
            vec![14],
        ]
    };


    html! {
        <div>
            <h1>{"Nonogram"}</h1>
            <p>{"Nonogram is a game where you have to fill in a grid with black or whites. Rows and columns tell you what how many blocks should be conseguetially painted black"}</p>
            <Grid2 
                cols={nonogram_example.cols} 
                rows={nonogram_example.rows}
                rows_rules={nonogram_example.rows_rules} 
                cols_rules={nonogram_example.cols_rules}
            />
        </div>
    }
}
