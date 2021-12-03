// use log::info;

use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

use crate::constant::{CardName, GameFlipCardResult, Status, KEY_BEST_SCORE};
use crate::helper::shuffle_cards;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DraftCard {
    pub id: String,
    pub name: CardName,
}
impl PartialEq for DraftCard {
    fn eq(&self, other: &DraftCard) -> bool {
        self.id == other.id && self.name == other.name
    }
    fn ne(&self, other: &DraftCard) -> bool {
        !self.eq(other)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Card {
    pub id: String,
    pub flipped: bool,
    pub name: CardName,
}

impl PartialEq<DraftCard> for &mut Card {
    fn eq(&self, other: &DraftCard) -> bool {
        self.id == other.id && self.name == other.name
    }
    fn ne(&self, other: &DraftCard) -> bool {
        !self.eq(other)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub unresolved_card_pairs: u8,
    pub best_score: u32,
    pub status: Status,
    pub cards: Vec<Card>,
    pub sec_past: u32,
    pub last_card: Option<DraftCard>,
}

impl State {
    pub fn reset() -> State {
        State {
            unresolved_card_pairs: 8,
            best_score: LocalStorage::get(KEY_BEST_SCORE).unwrap_or_else(|_| 9999),
            status: Status::READY,
            cards: shuffle_cards(),
            sec_past: 0,
            last_card: None,
        }
    }

    pub fn flip_card(&mut self, card: DraftCard) -> GameFlipCardResult {
        self.cards.iter_mut().filter(|c| c.eq(&card)).for_each(|c| {
            c.flipped = !c.flipped;
        });

        let last_card = self.last_card.clone();

        match last_card {
            None => {
                self.last_card = Some(card);
                GameFlipCardResult::GameContinue
            }
            Some(last_card) => {
                self.last_card = None;

                if card.id.ne(&last_card.id) && card.name.eq(&last_card.name) {
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

    pub fn try_to_start_game(&mut self) -> bool {
        if self.status == Status::READY {
            self.status = Status::PLAYING;
            return true;
        }
        false
    }

    pub fn flip_card_rollback(&mut self, cards: [DraftCard; 2]) {
        self.cards
            .iter_mut()
            .filter(|c| {
                cards.contains(
                    &(DraftCard {
                        id: c.id.clone(),
                        name: c.name,
                    }),
                )
            })
            .for_each(|c| {
                c.flipped = !c.flipped;
            });
    }

    pub fn sec_past_tick(&mut self) {
        self.sec_past = self.sec_past + 1;
    }

    pub fn save_best_score(&mut self) {
        (self.best_score > self.sec_past).then(|| LocalStorage::set(KEY_BEST_SCORE, self.sec_past));
        ()
    }
}
