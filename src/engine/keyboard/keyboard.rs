use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::window;
use web_sys::KeyboardEvent;
use yew::prelude::*;

#[hook]
pub fn use_on_key_pressed<F>(key: String, mut callback: F)
where
    F: FnMut() + 'static,
{

    use_effect_with_deps(
        move |_| {
            // Attach a keydown event listener to the document.
            let document = window().unwrap().document().unwrap();

            //let callback = callback.clone();

            let cb = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
                if event.key() == key {
                    callback();
                }
            });

            let res =
                document.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());

            if res.is_err() {
                log::error!("error: {:?}", res);
            }

            move || {
                document
                    .remove_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
                    .unwrap_throw()
            }
        },
        (),
    );
}
