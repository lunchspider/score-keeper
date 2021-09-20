mod pages;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum Route {
    #[to = "/create_table"]
    CreateTable,
    #[to = "/"]
    Home,
}


pub struct Model {
    link: ComponentLink<Self>,
}
impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_nav() }

                <main>
                    <Router<Route> render=Router::render(switch) />
                </main>
            </>
        }
    }
}
impl Model {
    fn view_nav(&self) -> Html {
        let Self {
            ref link,
            ..
        } = *self;

        html! {
            <nav>
                <div>
                    <h1>{ "Score Keeper" }</h1>

                </div>
                <div>
                    <nav class="navbar">
                        <li class="navbar_element"> <a><RouterAnchor<Route> route=Route::Home>
                            { "Home" }
                        </RouterAnchor<Route>>
                        </a></li>
                    </nav>
                     <li class="navbar_element"> <a>
                        <RouterAnchor<Route> route=Route::CreateTable>{ "create Table" }</RouterAnchor<Route>>
                        </a></li>
                </div>
            </nav>
        }
    }
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {<pages::App/>},
        Route::CreateTable => html! {<pages::CreateTable/>},
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<Model>();
    Ok(())
}

