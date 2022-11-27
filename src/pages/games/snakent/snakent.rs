use yew::prelude::*;

use crate::engine::{canvas::hooks::use_canvas, keyboard::keyboard::use_on_key_pressed};

#[function_component(Snakent)]
pub fn snakent() -> Html {
    let (node, cav) = use_canvas(|| (800, 800));

    use_effect(move || {
        cav.draw();
    });

    use_on_key_pressed("a".to_string(), move || {
        log::info!("a");
    });

    html! {
        <div>
            <h1>{"Snaken't"}</h1>
            <p>{"Snake is a famous game where you are a snake trying to eat stuff."}<b>{" Yes, this is the official description"}</b></p>
            <br/>
            <p>{"Of course I'm an evil person, so the Snaken't randomly block one of the directons when eating a square. Anyway I thought this was not enough, so instead of the arrows, you need to use VIM directions (h j k l)"}</p>
            <br/>
            {node}
        </div>
    }
}
