use {
    yew::prelude::*,
    yew::services::fetch::Request,
};

pub struct CreateTable {
    link: ComponentLink<Self>,
}

impl Component for CreateTable {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
        <>
        {"hello"}
        </>
        }
    }
}
