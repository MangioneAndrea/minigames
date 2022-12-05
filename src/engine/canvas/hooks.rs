use std::{cell::RefCell, rc::Rc};

use super::{actor::Actor, canvas::Canvas};
use yew::prelude::*;

#[hook]
pub fn use_canvas<F>(init_fn: F) -> (Html, Rc<RefCell<Canvas>>)
where
    F: FnOnce() -> (usize, usize, Vec<Actor>),
{
    let state = use_state(init_fn);
    let (width, height, initial_actors) = &*state;
    let mut cv = Rc::new(RefCell::new(Canvas::new(use_node_ref(), *height, *width)));

    initial_actors.iter().for_each(|actor| {
        RefCell::get_mut(&mut Rc::get_mut(&mut cv).unwrap()).register_actor(actor.clone());
    });

    (
        html!(
            <canvas ref={&Rc::get_mut(&mut cv).unwrap().get_mut().node_ref} width={width.to_string()} height={height.to_string()} class={"border-2"} />
        ),
        cv,
    )
}
