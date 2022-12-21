use crate::engine::entities::{actor::Actor, component::Component};

#[derive(Clone, PartialEq)]
pub struct Mover {
    pub x: f64,
    pub y: f64,
}

impl Component for Mover {
    fn clone_dyn(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }

    fn act(&self) -> dyn FnMut(&mut dyn Actor) {
        let x = self.x;
        let y = self.y;
        Box::new(move |actor| actor.push_by(x, y))
    }
}

impl Mover {
    pub fn new(x: f64, y: f64) -> Mover {
        Mover { x, y }
    }
}
