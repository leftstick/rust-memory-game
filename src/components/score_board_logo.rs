use yew::{html, Component, ComponentLink, Html, ShouldRender};

#[derive(Debug)]
pub struct Logo;

impl Component for Logo {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <h1 class="logo">
                <a href="https://github.com/leftstick/rust-memory-game" target="_blank">{"Memory"}</a>
            </h1>
        }
    }
}
