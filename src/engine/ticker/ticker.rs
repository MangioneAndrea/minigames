use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use yew::{hook, use_effect_with_deps};
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn cancelInterval(token: f64);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Interval {
    closure: Closure<dyn FnMut()>,
    token: f64,
}

impl Interval {
    pub fn new<F: 'static>(millis: u32, f: F) -> Interval
    where
        F: FnMut() + 'static,
    {
        // Construct a new closure.
        let closure = Closure::new(f);

        // Pass the closure to JS, to run every n milliseconds.
        let token = setInterval(&closure, millis);

        Interval { closure, token }
    }
}

// When the Interval is destroyed, cancel its `setInterval` timer.
impl Drop for Interval {
    fn drop(&mut self) {
        cancelInterval(self.token);
    }
}

#[hook]
pub fn use_interval<F>(interval: u32,  callback: F)
where
    F: FnMut() + 'static,
{

    use_effect_with_deps(
        move |_| {

            let iv = Interval::new(interval,  callback);
            log::info!("interval: {:?}", interval);

            move || {
                log::info!("interval: {:?}", interval);
                drop(iv);
            }
        },
        (),
    );
}
