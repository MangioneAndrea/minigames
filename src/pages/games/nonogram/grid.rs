use std::cmp;

use yew::prelude::*;

pub enum Message {
    Clicked(usize, usize),
}

#[derive(Clone, PartialEq)]
pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    pub grid: Vec<Vec<bool>>,
}
#[derive(Properties, Clone, PartialEq)]
pub struct GridProps {
    pub rows: usize,
    pub cols: usize,
    pub rows_rules: Vec<Vec<usize>>,
    pub cols_rules: Vec<Vec<usize>>,
}

#[function_component(Grid2)]
pub fn grid(props: &GridProps) -> Html {
    let gridb = use_state(|| {
        let mut grid = Vec::new();
        for _ in 0..props.rows {
            let mut row = Vec::new();
            for _ in 0..props.cols {
                row.push(false);
            }
            grid.push(row);
        }
        grid
    });

    let oncellclick = |grid: UseStateHandle<Vec<Vec<bool>>>, row: usize, col: usize| {
        Callback::from(move |_: MouseEvent| {
            grid.set({
                let mut grid = (*grid).clone();
                grid[row][col] = !grid[row][col];
                grid.to_vec()
            });
        })
    };

    let max_rows = {
        let mut max = 0;
        for row in &props.rows_rules {
            max = cmp::max(max, row.len());
        }
        max
    };

    let max_cols = {
        let mut max = 0;
        for col in &props.cols_rules {
            max = cmp::max(max, col.len());
        }
        max
    };

    log::info!("max_rows: {}", max_rows);
    log::info!("max_cols: {}", max_cols);

    html! {
        <div class={format!("border-4 grid grid-cols-{} w-fit", props.cols+max_rows)}>
            {
                (0..(max_rows+props.cols)*max_cols)
                    .map(|pos| html!{<div class="w-6 h-6 bg-green">{
                        if pos%(max_rows+props.cols)*max_cols <max_rows{
                            // empty angle
                            format!("")
                        }else{
                            let col_elem= pos/((max_rows+props.cols));
                            let col_index=pos%((max_rows+props.cols))-1;

                            if props.cols_rules[col_index].len() > col_elem{
                                format!("{}", props.cols_rules[col_index][col_elem])
                            }else{
                                format!("")
                            }
                        } }</div>})
                    .collect::<Html>()
            }
            { (0 .. (props).rows).map(|row|{

                (0..(max_rows))
                    .map(|_| html!{<div class="w-6 h-6 bg-green">{"0"}</div>})
                    .chain(
                        (0 .. props.cols).map(|col|{
                            html! {
                                <div class={format!("border margin w-6 h-6 bg-{}", if gridb[row][col] {"black"}else{"white"})} onclick={oncellclick(gridb.clone(),row.clone(), col.clone())}/>
                            }
                })).collect::<Html>()
            }).collect::<Html>() }
        </div>
    }
}