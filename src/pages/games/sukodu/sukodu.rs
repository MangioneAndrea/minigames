use yew::prelude::*;

use crate::pages::games::sukodu::cell::Cell;

#[function_component(Sukodu3x3)]
fn sukodu3x3() -> Html {
    html! {
        <div class={"border-2"}>
            <div class={"grid grid-cols-3 w-fit"}>
                <Cell number={1} can_edit={true} />
                <Cell number={2}/>
                <Cell number={3} />
            </div>
            <div class={"grid grid-cols-3 w-fit"}>
                <Cell number={4} />
                <Cell number={5} />
                <Cell number={6} />
            </div>
            <div class={"grid grid-cols-3 w-fit"}>
                <Cell number={7} />
                <Cell number={8} />
                <Cell number={9} />
            </div>
        </div>
    }
}

#[function_component(Sukodu9x9)]
fn sukodu9x9() -> Html {
    html! {
        <div class={"border-4 w-fit"}>
            <div class={"grid grid-cols-3 w-fit"}>
                <Sukodu3x3/>
                <Sukodu3x3/>
                <Sukodu3x3/>
            </div>
            <div class={"grid grid-cols-3 w-fit"}>
                <Sukodu3x3/>
                <Sukodu3x3/>
                <Sukodu3x3/>
            </div>
            <div class={"grid grid-cols-3 w-fit"}>
                <Sukodu3x3/>
                <Sukodu3x3/>
                <Sukodu3x3/>
            </div>
        </div>
    }
}

#[function_component(Sukodu)]
pub fn sukodu() -> Html {
    html! {
        <div>
            <h1>{"Sukodu"}</h1>
            <p>{"Sudoku is a game where you have to fill in a grid with numbers. This game just tries to confuse you by swapping the grid randomly (but still keeping the meaning!)"}</p>
            <Sukodu9x9/>
            </div>
    }
}
