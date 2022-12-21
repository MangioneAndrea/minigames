use super::actor::Actor;

pub trait Component {
    fn act(&self) -> dyn FnMut(&mut dyn Actor);
    fn clone_dyn(&self) -> Box<dyn Component>;
}

impl PartialEq for Box<dyn Component> {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }

    fn ne(&self, other: &Self) -> bool {
        self != other
    }
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}
