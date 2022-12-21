use web_sys::CanvasRenderingContext2d;

pub trait Actor {
    fn move_to(&mut self, x: f64, y: f64);
    fn push_by(&mut self, x: f64, y: f64);
    fn draw(&self, context: &CanvasRenderingContext2d);

    fn clone_dyn(&self) -> Box<dyn Actor>;
}

impl PartialEq for Box<dyn Actor> {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }

    fn ne(&self, other: &Self) -> bool {
        self != other
    }
}

impl Clone for Box<dyn Actor> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}
