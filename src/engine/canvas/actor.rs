use super::component::Component;

#[derive(Clone, PartialEq)]
pub struct Actor {
    pub x: f64,
    pub y: f64,
    pub components: Vec<Component>,
}

impl Actor {
    pub fn new() -> Actor {
        Actor {
            x: 0.,
            y: 0.,
            components: vec![],
        }
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn push_by(&mut self, x: f64, y: f64) {
        self.move_to(self.x + x, self.y + y)
    }
}
