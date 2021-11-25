// use log::info;
use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::components::{
    score_board_best_score::BestScore, score_board_logo::Logo, score_board_progress::GameProgress,
};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub unresolved_card_pairs: u8,
    pub best_score: u32,
}

pub struct ScoreBoard {
    props: Props,
}

impl Component for ScoreBoard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        if self.props.unresolved_card_pairs != _props.unresolved_card_pairs
            || self.props.best_score != _props.best_score
        {
            self.props.unresolved_card_pairs = _props.unresolved_card_pairs;
            self.props.best_score = _props.best_score;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="score-board">
                <Logo />
                <GameProgress unresolved_card_pairs={self.props.unresolved_card_pairs} />
                <BestScore best_score={self.props.best_score} />
            </div>
        }
    }
}
