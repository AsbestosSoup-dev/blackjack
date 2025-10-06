use rand::{rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use crate::core::card::Card;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Penetration {
    Full,
    P75,
    P50,
    P25,
    Custom(u8),
}

impl Penetration {
    pub fn from_percent(pct: u8) -> Self {
        if !(1..=100).contains(&pct) {
            return Self::P75;
        }
        match pct {
            100 => Self::Full,
            75 => Self::P75,
            50 => Self::P50,
            25 => Self::P25,
            _ => Self::Custom(pct),
        }
    }

    pub fn to_fraction(self) -> f32 {
        match self {
            Penetration::Full => 1.0,
            Penetration::P75 => 0.75,
            Penetration::P50 => 0.5,
            Penetration::P25 => 0.25,
            Penetration::Custom(pct) => pct as f32 / 100.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shoe {
    cards: Vec<Card>,
    position: usize,
    cut_index: usize,
    decks: u8,
}

impl Shoe {
    pub fn new(decks: u8) -> Self {
        Self::new_with_penetration(decks, Penetration::P75)
    }

    pub fn new_with_penetration(decks: u8, penetration: Penetration) -> Self {
        let cards_len = decks as usize * 52;
        let mut cards = Vec::with_capacity(cards_len);
        for _ in 0..decks {
            cards.extend(Card::standard_deck());
        }

        let mut rng = rng();
        cards.shuffle(&mut rng);

        // todo: validate penetration values via TUI

        Self {
            cards,
            position: 0,
            cut_index: (penetration.to_fraction() * cards_len as f32).ceil() as usize,
            decks,
        }
    }

    pub fn penetration(&self) -> f32 {
        self.cut_index as f32 / self.cards.len() as f32
    }

}
