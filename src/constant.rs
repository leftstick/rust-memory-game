use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

use crate::state::DraftCard;

pub const KEY_BEST_SCORE: &str = "rust.memory.game.best.score";

#[derive(Clone, Copy, Debug, EnumIter, Display, PartialEq, Serialize, Deserialize)]
pub enum CardName {
    EightBall,
    Kronos,
    BakedPotato,
    Dinosaur,
    Rocket,
    SkinnyUnicorn,
    ThatGuy,
    Zeppelin,
}

#[derive(Clone, Copy, Debug, EnumIter, Display, PartialEq, Serialize, Deserialize)]
pub enum Status {
    READY,
    PLAYING,
    PASSED,
}

#[derive(Clone, Debug, Display, PartialEq, Serialize, Deserialize)]
pub enum GameFlipCardResult {
    GameOver,
    GameContinue,
    CardRollback([DraftCard; 2]),
}

pub const RAW_CARDS: [CardName; 16] = [
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
