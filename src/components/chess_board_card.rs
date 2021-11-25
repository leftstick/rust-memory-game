use web_sys::MouseEvent;
use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::constant::CardName;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub id: String,
    pub name: CardName,
    pub flipped: bool,
    pub on_flip: Callback<Option<(String, CardName)>>,
}

pub struct ChessboardCard {
    props: Props,
}

impl ChessboardCard {
    fn get_link_by_cardname(&self) -> String {
        match self.props.name {
            CardName::EightBall => "public/8-ball.png",
            CardName::Kronos => "public/kronos.png",
            CardName::BakedPotato => "public/baked-potato.png",
            CardName::Dinosaur => "public/dinosaur.png",
            CardName::Rocket => "public/rocket.png",
            CardName::SkinnyUnicorn => "public/skinny-unicorn.png",
            CardName::ThatGuy => "public/that-guy.png",
            CardName::Zeppelin => "public/zeppelin.png",
        }
        .to_string()
    }
}

impl Component for ChessboardCard {
    type Message = ();
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props: _props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        if self.props.flipped != _props.flipped
            || self.props.id.ne(&_props.id)
            || self.props.name.ne(&_props.name)
        {
            self.props.flipped = _props.flipped;
            self.props.id = _props.id;
            self.props.name = _props.name;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let id = self.props.id.clone();
        let name = self.props.name;
        let flipped = self.props.flipped;

        let onclick = self.props.on_flip.reform(move |e: MouseEvent| {
            e.stop_propagation();

            if flipped {
                None
            } else {
                Some((id.clone(), name))
            }
        });

        html! {
            <div class="chess-board-card-container">
                <div class={classes!("card", self.props.flipped.then(|| "flipped"))} onclick={onclick}>
                    <img class="front" src={self.get_link_by_cardname()} />
                    <img class="back" src="public/back.png" />
                </div>
            </div>
        }
    }
}
