use yew::prelude::*;

use super::core::{Grid, Square};
use crate::pages::games::sukodu::cell::Cell;

#[function_component(Sukodu9x9)]
fn sukodu9x9() -> Html {
    let grid = use_state(|| {
        Grid::new(vec![
            vec![
                Square::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
                Square::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
                Square::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            ],
            vec![
                Square::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
                Square::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
                Square::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            ],
            vec![
                Square::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
                Square::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
                Square::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            ],
        ])
    });

    html! {
        <div class={"border-4 w-fit"}>
        {
            grid.squares.iter().map(|row| {
                html!{
                    <div class={"grid grid-cols-3 w-fit"}>
                        {
                            row.iter().map(|square| {
                                html!{
                                    <div class={"border-2"}>
                                        {
                                            square.rows.iter().map(|row| {
                                                html!{
                                                    <div class={"grid grid-cols-3 w-fit"}>
                                                        {
                                                            row.iter().map(|cell| {
                                                                html!{
                                                                    <div class={"border w-6 h-6 text-center"}>
                                                                    {cell}
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
            <p>{"Of course I'm an evil person, so the Sukodu randomly block one of the directons when eating a square. Anyway I thought this was not enough, so instead of the arrows, you need to use VIM directions (h j k l)"}</p>
            <br/>
            <Sukodu9x9/>
            </div>
    }
}
