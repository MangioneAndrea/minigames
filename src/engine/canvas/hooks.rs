use super::{canvas::Canvas, actor::Actor};
use yew::prelude::*;

#[hook]
pub fn use_canvas<F>(init_fn: F) -> (Html, Canvas)
where
    F: FnOnce() -> (usize, usize, Vec<Actor>),
{
    let state = use_state(init_fn);
    let (width, height, initial_actors) = &*state;
    let mut cv = Canvas::new(use_node_ref(), *height, *width);

    initial_actors.iter().for_each(|actor| {
        cv.register_actor(actor.clone());
    });

    (
        html!(
            <canvas ref={&cv.node_ref} width={width.to_string()} height={height.to_string()} class={"border-2"} />
        ),
        cv,
    )
}
