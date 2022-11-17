use yew::prelude::*;

pub struct Cell {
    props: Props,
   // class: &str,
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
        let mut class: String = "cell border w-8 h-8 text-center".to_string();

        if !ctx.props().can_edit.unwrap_or(false) {
            class += "text-gray-500 ";
        }

        Self {
            props: ctx.props().clone(),
            //class: class.as_str(),
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div /*class={self.class}*/>
                {self.props.number}
            </div>
        }
    }
}
