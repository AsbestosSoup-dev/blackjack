use serde::{Deserialize, Serialize};

use crate::core::payout::BlackjackPayout;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rules {
    pub min_bet: u32,
    pub max_bet: u32,
    pub starting_credits: u32,
    pub blackjack_payout: BlackjackPayout,
    pub num_decks: u8,
    pub split_limit: u8,
    pub dealer_hits_soft_17: bool,
    pub surrender_allowed: bool,
    pub resplit_aces_allowed: bool,
    pub hit_split_aces_allowed: bool,
    pub double_after_split_allowed: bool,

    // core side bets
    pub insurance_enabled: bool,
    pub perfect_pairs_enabled: bool,
    pub twenty_one_plus_3_enabled: bool,
    pub royal_match_enabled: bool,
    // high-variance side bets
    // pub lucky_ladies_enabled: bool,
    // pub bust_it_enabled: bool,
    // pub super_7s_enabled: bool,
    // pub lucky_lucky_enabled: bool,
    // pub pair_squared_enabled: bool,

    // specialty side bets
    // pub blackjack_spin_enabled: bool,
    // pub match_the_dealer_enabled: bool,
    // pub buster_blackjack_enabled: bool,
    // pub bet_the_set_enabled: bool,
    // pub top_3_enabled: bool,
    // pub hot_3_enabled: bool,
    // pub kings_bounty_enabled: bool,

    // progressive jackpots
    // pub caribbean_21_enabled: bool,
    // pub blazing_7s_enabled: bool,
}

impl Rules {
    pub fn new() -> Self {
        Self {
            min_bet: 10,
            max_bet: 500,
            starting_credits: 1000,
            blackjack_payout: BlackjackPayout::Standard,
            num_decks: 6,
            split_limit: 1,
            dealer_hits_soft_17: false,
            surrender_allowed: false,
            resplit_aces_allowed: false,
            hit_split_aces_allowed: false,
            double_after_split_allowed: true,
            insurance_enabled: false,
            perfect_pairs_enabled: false,
            twenty_one_plus_3_enabled: false,
            royal_match_enabled: false,
        }
    }
}

impl Default for Rules {
    fn default() -> Self {
        Self::new()
    }
}
