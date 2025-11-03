use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)] // No Copy, actions are consumed.
pub enum Action {
    Leave,
    Spectate,

    Bet { amount: u32 },
    BetInsurance { amount: u32 },
    BetPerfectPairs { amount: u32 },
    BetTwentyOnePlus3 { amount: u32 },
    BetRoyalMatch { amount: u32 },

    Hit,
    Stand,
    Double,
    Split,
    Surrender,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerAction {
    pub player_id: Uuid,
    pub action: Action,
}

impl PlayerAction {
    pub fn new(player_id: Uuid, action: Action) -> Self {
        Self { player_id, action }
    }
}
