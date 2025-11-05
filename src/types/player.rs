use crate::core::hand::Hand;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub credits: u32,
    pub is_spectator: bool,
    pub is_bot: bool,
    pub is_host: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerHand {
    pub player_id: Uuid,
    pub hand: Hand,
    pub bet: u32,
    pub insurance_bet: u32,
    pub perfect_pairs_bet: u32,
    pub twenty_one_plus_3_bet: u32,
    pub royal_match_bet: u32,
    // more side bets coming soon
}

impl Player {
    pub fn new(id: Uuid, name: String, starting_credits: u32, is_bot: bool) -> Self {
        Self {
            id,
            name,
            credits: starting_credits,
            is_spectator: false,
            is_bot,
            is_host: false,
        }
    }
}

impl PlayerHand {
    pub fn new(player_id: Uuid, bet: u32) -> Self {
        Self {
            player_id,
            hand: Hand::new(),
            bet,
            insurance_bet: 0,
            perfect_pairs_bet: 0,
            twenty_one_plus_3_bet: 0,
            royal_match_bet: 0,
        }
    }
}
