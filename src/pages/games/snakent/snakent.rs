use crate::engine::{
    entities::{actor::Actor, actors::square::Square, components::mover::Mover},
    ticker::ticker::use_interval,
};
use yew::prelude::*;

use crate::engine::{canvas::hooks::use_canvas, keyboard::keyboard::use_on_key_pressed};

const STEP: f64 = 40.;

fn get_actors() -> Vec<Box<dyn Actor>> {
    let mut actors: Vec<Box<dyn Actor>> = vec![];
    let mut player = Square::new(STEP, STEP);
    player.components.push(Box::new(Mover::new(10., 0.)));
    actors.push(Box::new(player));
    actors
}

#[function_component(Snakent)]
pub fn snakent() -> Html {
    let (node, cav) = use_canvas(|| (800, 800, get_actors()));

    use_interval(1000, {
        let cav = cav.clone();
        move || {
            for ele in &mut cav.borrow_mut().actors {
                ele.on_tick();
            }
            cav.borrow_mut().draw();
        }
    });

    use_on_key_pressed("l".to_string(), {
        let cav = cav.clone();
        move || {
            cav.borrow_mut().actors[0].push_by(10., 0.);
            cav.borrow_mut().draw();
        }
    });

    use_on_key_pressed("h".to_string(), {
        let cav = cav.clone();
        move || {
            cav.borrow_mut().actors[0].push_by(-10., 0.);
            cav.borrow_mut().draw();
        }
    });

    use_on_key_pressed("j".to_string(), {
        let cav = cav.clone();
        move || {
            cav.borrow_mut().actors[0].push_by(0., 10.);
            cav.borrow_mut().draw();
        }
    });

    use_on_key_pressed("k".to_string(), {
        let cav = cav.clone();
        move || {
            cav.borrow_mut().actors[0].push_by(0., -10.);
            cav.borrow_mut().draw();
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
