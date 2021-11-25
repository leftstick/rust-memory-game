// use log::info;

use gloo::storage::{LocalStorage, Storage};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use crate::constant::{CardName, GameFlipCardResult, Status, KEY_BEST_SCORE};
use crate::helper::shuffle_cards;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Card {
    pub id: String,
    pub flipped: bool,
    pub name: CardName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub unresolved_card_pairs: u8,
    pub best_score: u32,
    pub status: Status,
    pub cards: Vec<Card>,
    pub sec_past: u32,
    pub last_card: Option<(String, CardName)>,
}

impl State {
    pub fn reset() -> State {
        let mut raw_cards = vec![
            CardName::EightBall,
            CardName::Kronos,
            CardName::BakedPotato,
            CardName::Dinosaur,
            CardName::Rocket,
            CardName::SkinnyUnicorn,
            CardName::ThatGuy,
            CardName::Zeppelin,
            CardName::EightBall,
            CardName::Kronos,
            CardName::BakedPotato,
            CardName::Dinosaur,
            CardName::Rocket,
            CardName::SkinnyUnicorn,
            CardName::ThatGuy,
            CardName::Zeppelin,
        ];

        raw_cards.shuffle(&mut thread_rng());

        State {
            unresolved_card_pairs: 8,
            best_score: LocalStorage::get(KEY_BEST_SCORE).unwrap_or_else(|_| 9999),
            status: Status::READY,
            cards: shuffle_cards(),
            sec_past: 0,
            last_card: None,
        }
    }

    pub fn flip_card(&mut self, card: (String, CardName)) -> GameFlipCardResult {
        for c in self.cards.iter_mut() {
            if card.0.eq(&c.id) && card.1.eq(&c.name) {
                c.flipped = !c.flipped;
            }
        }

        let last_card = self.last_card.clone();

        match last_card {
            None => {
                self.last_card = Some(card);
                GameFlipCardResult::GameContinue
            }
            Some(last_card) => {
                self.last_card = None;

                if card.0.ne(&last_card.0) && card.1.eq(&last_card.1) {
                    self.unresolved_card_pairs = self.unresolved_card_pairs - 1;

                    if self.unresolved_card_pairs == 0 {
                        self.status = Status::PASSED;

                        GameFlipCardResult::GameOver
                    } else {
                        GameFlipCardResult::GameContinue
                    }
                } else {
                    GameFlipCardResult::CardRollback([last_card, card])
                }
            }
        }
    }

    pub fn flip_card_rollback(&mut self, cards: [(String, CardName); 2]) {
        for c in self.cards.iter_mut() {
            if cards[0].0.eq(&c.id) && cards[0].1.eq(&c.name) {
                c.flipped = !c.flipped;
            }
            if cards[1].0.eq(&c.id) && cards[1].1.eq(&c.name) {
                c.flipped = !c.flipped;
            }
        }
    }

    pub fn sec_past_tick(&mut self) {
        self.sec_past = self.sec_past + 1;
    }

    pub fn save_best_score(&mut self) {
        if self.best_score > self.sec_past {
            let _ = LocalStorage::set(KEY_BEST_SCORE, self.sec_past);
        }
    }
}
