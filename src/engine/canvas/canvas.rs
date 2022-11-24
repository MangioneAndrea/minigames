use js_sys::Object;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use yew::{NodeRef, use_node_ref};

#[derive(Clone, PartialEq)]
pub struct Canvas {
    entities: Vec<Object>,
    pub canvas: Option<HtmlCanvasElement>,
    pub node_ref: NodeRef,
}

impl Canvas {
    pub fn new(node_ref: NodeRef) -> Canvas {
        Canvas {
            entities: vec![],
            canvas: None,
            node_ref: node_ref,
        }
    }

    pub fn draw(&self) {
        let context = self
            .node_ref
            .cast::<HtmlCanvasElement>()
            .unwrap()
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.fill_rect(0., 0., 100., 100.);

        log::info!("Drawing");

        context.stroke();
    }

    pub fn clear(&self) {
        // ...
    }
}
