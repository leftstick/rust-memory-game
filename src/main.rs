mod components;
mod constant;
mod helper;
mod state;

use yew::{html, Component, ComponentLink, Html, ShouldRender};

use gloo::timers::callback::{Interval, Timeout};

use components::{
    chess_board_card::ChessboardCard, game_status_board::GameStatusBoard, score_board::ScoreBoard,
};

use constant::{GameFlipCardResult, Status};
use state::{DraftCard, State};

pub enum Msg {
    FlipCard(Option<DraftCard>),
    RollbackCards([DraftCard; 2]),
    GameReset,
    SecPast,
    GameOver,
}

#[derive(Debug)]
pub struct Model {
    state: State,
    link: ComponentLink<Self>,
    sec_past_timer: Option<Interval>,
    flip_back_timer: Option<Timeout>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State::reset();

        Self {
            state,
            link,
            sec_past_timer: None,
            flip_back_timer: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        match _msg {
            Msg::FlipCard(raw_card) => match raw_card {
                None => false,
                Some(card) => {
                    // try to start game
                    if self.state.status == Status::READY {
                        self.state.status = Status::PLAYING;
                        let link = self.link.clone();
                        self.sec_past_timer =
                            Some(Interval::new(1000, move || link.send_message(Msg::SecPast)));
                    }
                    let result = self.state.flip_card(card);
                    match result {
                        GameFlipCardResult::CardRollback(cards) => {
                            let link = self.link.clone();
                            self.flip_back_timer = Some(Timeout::new(1000, move || {
                                link.send_message(Msg::RollbackCards(cards))
                            }));
                        }
                        GameFlipCardResult::GameOver => {
                            self.link.send_message(Msg::GameOver);
                        }
                        GameFlipCardResult::GameContinue => {}
                    }
                    true
                }
            },
            Msg::RollbackCards(cards) => {
                self.state.flip_card_rollback(cards);
                true
            }
            Msg::GameReset => {
                self.state = State::reset();
                self.flip_back_timer = None;
                self.sec_past_timer = None;
                true
            }
            Msg::GameOver => {
                self.sec_past_timer = None;
                self.flip_back_timer = None;
                self.state.save_best_score();
                true
            }
            Msg::SecPast => {
                self.state.sec_past_tick();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="game-panel">
                <ScoreBoard unresolved_card_pairs={self.state.unresolved_card_pairs} best_score={self.state.best_score} />
                <div class="chess-board">
                    { for self.state.cards.iter().map(|card|
                        html! {
                            <ChessboardCard id={card.id.clone()} name={card.name} flipped={card.flipped} on_flip={self.link.callback(|card| Msg::FlipCard(card))}/>
                        }
                    ) }
                </div>
                <GameStatusBoard sec_past={self.state.sec_past} status={self.state.status} on_reset={self.link.callback(|_| Msg::GameReset)}/>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
