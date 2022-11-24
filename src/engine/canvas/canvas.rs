use js_sys::Object;
use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;
pub struct Canvas {
    drawings: Object,
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas {
            drawings: Object::new(),
        }
    }

    pub fn draw(&self, drawable: &HtmlCanvasElement) {
        let context= drawable
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
