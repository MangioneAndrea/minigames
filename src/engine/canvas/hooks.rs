use super::canvas::Canvas;
use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[hook]
pub fn use_canvas<F>(init_fn: F) -> (Html)
where
    F: FnOnce() -> (usize, usize),
{
    let state = use_state(init_fn);
    let canvas_ref = use_node_ref();
    let canvas_node = canvas_ref.cast::<HtmlCanvasElement>();
    let cv = Canvas::new();

    let onclick = Callback::from(move |e: MouseEvent| {
        cv.draw(&e.target().unwrap().dyn_into::<HtmlCanvasElement>().unwrap());
    });


    let (width, height) = *state;
    (html!(
        <canvas {onclick} ref={canvas_ref} width={width.to_string()} height={height.to_string()} class={"border-2"} />
    ))
}
