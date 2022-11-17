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

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut grid = Vec::new();
        for _ in 0..rows {
            let mut row = Vec::new();
            for _ in 0..cols {
                row.push(false);
            }
            grid.push(row);
        }
        Self { rows, cols, grid }
    }

    pub fn toggle(&mut self, row: usize, col: usize) {
        self.grid[row][col] = !self.grid[row][col];
    }
}

impl Component for Grid {
    type Message = Message;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self::new(10, 10)
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("hey");
        match msg {
            Message::Clicked(row, col) => {
                self.toggle(row, col);
                true
            },
            _ => {
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={format!("border-4 grid grid-cols-{} w-fit", self.cols)}>
                { (0 .. (self).cols).map(|col|{
                    (0 .. self.rows).map(|row|{
                        html! {
                            <div class={format!("border margin w-6 h-6 bg-{}", if self.grid[col][row] {"black"}else{"white"})} onclick={ctx.link().callback(move |_| Message::Clicked(col,row))}/>
                        }
                    }).collect::<Html>()
                }).collect::<Html>() }
            </div>
        }
    }
}
