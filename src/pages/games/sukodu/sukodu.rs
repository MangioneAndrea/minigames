use yew::prelude::*;

use super::core::{Grid, Square};

#[function_component(Sukodu9x9)]
fn sukodu9x9() -> Html {
    let grid = use_state(|| {
        Grid::new(vec![
            vec![
                Square::new(vec![vec![0, 0, 0], vec![6, 8, 0], vec![1, 9, 0]]),
                Square::new(vec![vec![2, 6, 0], vec![0, 7, 0], vec![0, 0, 4]]),
                Square::new(vec![vec![7, 0, 1], vec![0, 9, 0], vec![5, 0, 0]]),
            ],
            vec![
                Square::new(vec![vec![8, 2, 0], vec![0, 0, 4], vec![0, 5, 0]]),
                Square::new(vec![vec![1, 0, 0], vec![6, 0, 2], vec![0, 0, 3]]),
                Square::new(vec![vec![0, 4, 0], vec![9, 0, 0], vec![0, 2, 8]]),
            ],
            vec![
                Square::new(vec![vec![0, 0, 9], vec![0, 4, 0], vec![7, 0, 3]]),
                Square::new(vec![vec![3, 0, 0], vec![0, 5, 0], vec![0, 1, 8]]),
                Square::new(vec![vec![0, 7, 4], vec![0, 3, 6], vec![0, 0, 0]]),
            ],
        ])
    });

    let onclick = |square_row: usize, square_column: usize, cell_row: usize, cell_column: usize| {
        let grid = grid.clone();
        Callback::from(move |_| {
            let gr = &mut (*grid).clone();
            gr.squares[square_row][square_column].increase_at(cell_row, cell_column);
            gr.squares[square_row][square_column].check_and_mark();
            gr.check_and_mark(square_row * 3 + cell_row, square_column * 3 + cell_column);
            grid.set(gr.to_owned());
        })
    };

    html! {
        <div class={"border-4 w-fit border-gray-600"}>
        {
            grid.squares.iter().enumerate().map(|(r_index, row)| {
                html!{
                    <div class={"grid grid-cols-3 w-fit"}>
                        {
                            row.iter().enumerate().map(|(s_index, square)| {
                                html!{
                                    <div class={"border-2 border-gray-400"}>
                                        {
                                            square.rows.iter().enumerate().map(|(sr_index, row)| {
                                                html!{
                                                    <div class={"grid grid-cols-3 w-fit"}>
                                                        {
                                                            row.iter().enumerate().map(|(c_index, cell)| {
                                                                html!{
                                                                    <div onclick={onclick(r_index, s_index, sr_index, c_index)} class={format!("border w-6 h-6 text-center align-middle {}", cell.format())}>
                                                                    {if cell.value > 0 {
                                                                        format!("{}", cell.value)
                                                                    } else {
                                                                        format!("")
                                                                    }}
                                                                    </div>
                                                                }
                                                            }).collect::<Html>()
                                                        }
                                                    </div>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                }
            }).collect::<Html>()
        }
        </div>
    }
}

#[function_component(Sukodu)]
pub fn sukodu() -> Html {
    html! {
        <div>
            <h1>{"Sukodu"}</h1>
            <p>{"Sudoku is a game where you have to fill in a grid with numbers. You are not allowed to reuse the same number within the same 3x3 square, nor within the same 9x1 or 1x9 line"}</p>
            <br/>
            <p>{"Of course I'm an evil person, so the Sudoku randomly switches the squares in the grid. Don't worry the logic is the same, and if it was correct before, it is correct after the swap."}<br/>{"...only problem is, that you can only increase one cell by 1 when clicking on it. Have fun!"}</p>
            <br/>
            <Sukodu9x9/>
            </div>
    }
}
