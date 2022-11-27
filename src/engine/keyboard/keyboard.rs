use js_sys::Function;
use web_sys::KeyboardEvent;
use web_sys::window;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;

#[hook]
pub fn use_on_key_pressed<F>((key, callback): (String, F)) -> ()
where
    F: Fn(),
{
    let state = use_state(|| Function::new_no_args(callback));

    use_effect(move || {
        // Attach a keydown event listener to the document.
        let document = window().unwrap().document().unwrap();
        document.add_event_listener_with_callback("keydown", |event: KeyboardEvent| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            // In this example, we just log the event, but obviously this is where your code goes.
            log::info!("event: {:?}", event);
        });

        || document.remove_event_listener_with_callback("keydown", state.as_ref().unwrap_throw()).unwrap_throw()
    });
}
