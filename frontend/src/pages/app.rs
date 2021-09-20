use {
    yew::prelude::*,
    yew::services::fetch::Request,
};

pub struct App {
    link: ComponentLink<Self>,
}

impl Component for App {
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
        {"hello want to create your own score keeper?"}
        </>
        }
    }
}

