use crate::core::card::Card;
use crate::error::{ConfigError, ConfigResult, GameError, GameResult};
use rand::SeedableRng;
use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shoe {
    cards: Vec<Card>,
    top_position: usize,
    cut_position: usize,
    num_decks: u8,
    rng: ChaCha8Rng,
}

impl Shoe {
    pub fn new(num_decks: u8, cut_position: usize, seed: Option<u64>) -> ConfigResult<Self> {
        if num_decks == 0 {
            return Err(ConfigError::InvalidDecks(num_decks));
        }

        let total_cards = num_decks as usize * 52;

        let mut cards = Vec::with_capacity(total_cards);
        for _ in 0..num_decks {
            cards.extend(Card::standard_deck());
        }

        let mut rng = match seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => ChaCha8Rng::from_os_rng(),
        };

        cards.shuffle(&mut rng);

        if 0 == cut_position || cut_position > total_cards {
            return Err(ConfigError::InvalidCutPosition(cut_position, total_cards));
        }

        Ok(Self {
            cards,
            num_decks,
            top_position: 0,
            cut_position,
            rng,
        })
    }

    pub fn deal(&mut self) -> GameResult<Card> {
        if self.top_position >= self.cut_position {
            return Err(GameError::ShoeNeedsReshuffling);
        }

        let card = self.cards[self.top_position];
        self.top_position += 1;
        Ok(card)
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut self.rng);
        self.top_position = 0
    }
}
