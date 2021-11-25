use yew::prelude::*;
use yew::{html, Component, Html};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub unresolved_card_pairs: u8,
}

pub struct GameProgress {
    props: Props,
}

impl Component for GameProgress {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        if self.props.unresolved_card_pairs != _props.unresolved_card_pairs {
            self.props.unresolved_card_pairs = _props.unresolved_card_pairs;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="game-progress">
                <span>{"Cards not Matched"}</span>
                <h2>{ self.props.unresolved_card_pairs }</h2>
            </div>
        }
    }
}
