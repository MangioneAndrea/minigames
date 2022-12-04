use crate::engine::canvas::actor::Actor;
use yew::prelude::*;

use crate::engine::{canvas::hooks::use_canvas, keyboard::keyboard::use_on_key_pressed};

#[function_component(Snakent)]
pub fn snakent() -> Html {
    let (node, mut cav) = use_canvas(|| (800, 800, vec![Actor::new()]));

    use_on_key_pressed("l".to_string(), {
        move || {
            let actor = &mut cav.actors[0];
            actor.push_by(10., 0.);
            (&cav).draw();
        }
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
