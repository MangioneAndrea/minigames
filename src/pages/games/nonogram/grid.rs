use super::core::NonogramCore;
use super::core::Rule;
use std::cmp;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct GridProps {
    pub rows: usize,
    pub cols: usize,
    pub rows_rules: Vec<Rule>,
    pub cols_rules: Vec<Rule>,
}

#[function_component(Grid)]
pub fn grid(props: &GridProps) -> Html {
    let gridb = use_state(|| NonogramCore::new(props.rows, props.cols));

    let oncellclick = |grid: UseStateHandle<NonogramCore>, row: usize, col: usize| {
        Callback::from(move |_: MouseEvent| {
            grid.set((*grid).change_cell(row, col));
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

    let total_width = max_rows + props.cols;

    let empty_cell = || html! {<div class="w-6 h-6 text-center"/>};
    let colored_num_cell = |num: usize, color: String| html! {<div class={format!("w-6 h-6 text-center {}", color)}>{num}</div>};
    let num_cell = |num: usize| colored_num_cell(num, String::from(""));

    html! {
        <div class={format!("grid w-fit")} style={format!("grid-template-columns:repeat({}, minmax(0,1fr));", props.cols+max_rows)}>
            {
                (0..total_width*max_cols)
                    .map(|pos| {
                        let column_index = pos % total_width;

                        if column_index < max_rows{
                            // empty angle
                            empty_cell()
                        }else{
                            let col_elem= pos/total_width;
                            let col_index=column_index-max_rows;

                            let empty_cells = max_cols - props.cols_rules[col_index].len();


                            if col_elem < empty_cells{
                                empty_cell()
                            }else{
                                let is_valid=props.cols_rules[col_index].is_full_and_valid(gridb.rotated[col_index].clone());
                                let row_index = col_elem - empty_cells;
                                let num = props.cols_rules[col_index][row_index];
                                colored_num_cell(num, if is_valid {String::from("bg-lime-300")} else {String::from("")})
                            }

                        }
                    }).collect::<Html>()
            }
            { (0 .. (props).rows).map(|row|{
                let is_valid=props.rows_rules[row].is_full_and_valid(gridb[row].clone());

                (0..(max_rows))
                    .map(|mr| {
                        let empty_cells = max_rows - props.rows_rules[row].len();
                        if mr < empty_cells{
                            empty_cell()
                        }else{
                            let row_index = mr - empty_cells;
                            let num = props.rows_rules[row][row_index];
                            colored_num_cell(num, if is_valid {String::from("bg-lime-300")} else {String::from("")})
                        }
                    })
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
