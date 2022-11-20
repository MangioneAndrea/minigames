use yew::prelude::*;

#[function_component(Snakent)]
pub fn snakent() -> Html {
    html! {
        <div>
            <h1>{"Snaken't"}</h1>
            <p>{"Snake is a famous game where you are a snake trying to eat stuff."}<b>{" Yes, this is the official description"}</b></p>
            <br/>
            <p>{"Of course I'm an evil person, so the Snaken't randomly block one of the directons when eating a square. Anyway I thought this was not enough, so instead of the arrows, you need to use VIM directions (h j k l)"}</p>
            <br/>
        </div>
    }
}
