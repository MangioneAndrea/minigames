use super::actor::Actor;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::NodeRef;

#[derive(Clone, PartialEq)]
pub struct Canvas {
    pub actors: Vec<Actor>,
    width: usize,
    height: usize,
    pub canvas: Option<HtmlCanvasElement>,
    pub node_ref: NodeRef,
    context: Option<CanvasRenderingContext2d>,
}

impl AsRef<Canvas> for Canvas {
    fn as_ref(&self) -> &Canvas {
        self
    }
}

impl Canvas {
    pub fn new(node_ref: NodeRef, height: usize, width: usize) -> Canvas {
        Canvas {
            actors: vec![],
            width,
            height,
            canvas: None,
            node_ref: node_ref,
            context: Option::None,
        }
    }

    pub fn register_actor(&mut self, actor: Actor) {
        self.actors.push(actor);
    }

    fn assert_context(&self) -> CanvasRenderingContext2d {
        return self
            .node_ref
            .cast::<HtmlCanvasElement>()
            .unwrap()
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
    }

    pub fn draw(&self) {
        let context = self.assert_context();

        self.clear();

        for ele in &self.actors {
            context.fill_rect(ele.x, ele.y, 100., 100.);
        }

        log::info!("Drawing");

        context.stroke();
    }

    pub fn clear(&self) {
        self.assert_context()
            .clear_rect(0., 0., self.width as f64, self.height as f64);
    }
}
