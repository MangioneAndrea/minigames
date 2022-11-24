use super::component::Component;

pub struct Actor {
    pub x: f64,
    pub y: f64,
    pub components: Vec<Component>,
}