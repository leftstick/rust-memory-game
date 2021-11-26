// use log::info;
use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::constant::Status;

#[derive(Clone, Properties)]
pub struct Props {
    pub status: Status,
    pub sec_past: u32,
    pub on_reset: Callback<()>,
}

impl PartialEq for Props {
    fn eq(&self, other: &Props) -> bool {
        self.status == other.status && self.sec_past == other.sec_past
    }

    fn ne(&self, other: &Props) -> bool {
        !self.eq(other)
    }
}

pub struct GameStatusBoard {
    props: Props,
}

impl GameStatusBoard {
    fn get_content(&self) -> Html {
        let onclick = self.props.on_reset.reform(move |e: MouseEvent| {
            e.stop_propagation();
            e.prevent_default();

            ()
        });

        match self.props.status {
            Status::READY => html! {
                <span >{"Ready"}</span>
            },
            Status::PLAYING => html! {
                <span >{"Playing"}</span>
            },
            Status::PASSED => html! {
                <a onclick={onclick}>{"Play again"}</a>
            },
        }
    }
}

impl Component for GameStatusBoard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        if self.props.ne(&_props) {
            self.props.status = _props.status;
            self.props.sec_past = _props.sec_past;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="game-status-container">
                {self.get_content()}
                <span class="sec-past">{ self.props.sec_past}{" s"}</span>
            </div>
        }
    }
}
