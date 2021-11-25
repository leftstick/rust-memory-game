use yew::prelude::*;
use yew::{html, Component, Html};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub best_score: u32,
}

pub struct BestScore {
    props: Props,
}

impl Component for BestScore {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        if self.props.best_score != _props.best_score {
            self.props.best_score = _props.best_score;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="best-score">
                <span>{"Highest Record"}</span>
                <h2>{ self.props.best_score }</h2>
            </div>
        }
    }
}
