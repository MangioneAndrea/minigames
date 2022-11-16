use yew::prelude::*;

pub struct Cell {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub number: Option<i8>,
    pub can_edit: Option<bool>,
}
impl Component for Cell {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self{
            props: ctx.props().clone(),
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div class="cell">
                {self.props.number}
            </div>
        }
    }
}
