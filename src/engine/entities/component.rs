
pub trait Component {
    fn on_construction(&self);
    fn on_tick(&self);
}
