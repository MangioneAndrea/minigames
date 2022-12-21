use web_sys::CanvasRenderingContext2d;

use super::super::actor::Actor;

#[derive(Clone, PartialEq)]
pub struct Square {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Square {
    pub fn new(width: f64, height: f64) -> Square {
        Square {
            x: 0.,
            y: 0.,
            width,
            height,
        }
    }
}

impl Actor for Square {
    fn move_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    fn push_by(&mut self, x: f64, y: f64) {
        self.move_to(self.x + x, self.y + y)
    }

    fn draw(&self, context: &CanvasRenderingContext2d) {
        context.fill_rect(self.x, self.y, self.width, self.height);
    }

    fn clone_dyn(&self) -> Box<dyn Actor> {
        Box::new(self.clone())
    }
}
