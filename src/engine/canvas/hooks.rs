use super::canvas::Canvas;
use yew::prelude::*;

#[hook]
pub fn use_canvas<F>(init_fn: F) -> (Html, Canvas)
where
    F: FnOnce() -> (usize, usize),
{
    let state = use_state(init_fn);
    let  canvas = Canvas::new(use_node_ref());

    let (width, height) = *state;
    (
        html!(
            <canvas ref={&canvas.node_ref} width={width.to_string()} height={height.to_string()} class={"border-2"} />
        ),
        canvas,
    )
}
