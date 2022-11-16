use crate::pages::games::sukodu::sukodu::Sukodu;
use yew::prelude::*;

#[function_component(Games)]
pub fn games() -> Html {
    html! {
        <div>
            <Sukodu/>
        </div>
    }
}
