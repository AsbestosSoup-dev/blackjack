use crate::core::hand::Hand;
use crate::core::rules::Rules;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum BlackjackPayout {
    Standard, // 3:2
    Vegas,    // 6:5
    Custom { numerator: u32, denominator: u32 },
}

pub fn calculate_payout(bet: u32, player_hand: &Hand, dealer_hand: &Hand, rules: &Rules) -> i64 {
    if player_hand.is_bust() {
        return -(bet as i64);
    }

    if dealer_hand.is_bust() {
        return bet as i64;
    }

    let player_hand_value = player_hand.value();
    let dealer_hand_value = dealer_hand.value();

    match player_hand_value.cmp(&dealer_hand_value) {
        std::cmp::Ordering::Greater => {
            if player_hand.is_blackjack() {
                calculate_payout_for_blackjack(bet, rules.blackjack_payout)
            } else {
                bet as i64
            }
        }
        std::cmp::Ordering::Less => -(bet as i64),
        std::cmp::Ordering::Equal => 0,
    }
}

fn calculate_payout_for_blackjack(bet: u32, blackjack_payout: BlackjackPayout) -> i64 {
    match blackjack_payout {
        BlackjackPayout::Standard => (bet * 3 / 2) as i64,
        BlackjackPayout::Vegas => (bet * 6 / 5) as i64,
        BlackjackPayout::Custom {
            numerator,
            denominator,
        } => (bet * numerator / denominator) as i64,
    }
}

pub fn calculate_insurance_payout(insurance_bet: u32, dealer_hand: &Hand) -> i64 {
    if dealer_hand.is_blackjack() {
        (insurance_bet as u64 * 2) as i64
    } else {
        -(insurance_bet as i64)
    }
}

pub fn calculate_perfect_pairs_payout(side_bet: u32, player_hand: &Hand) -> i64 {
    if player_hand.cards().len() != 2 {
        return -(side_bet as i64);
    }

    unimplemented!("Perfect Pairs - v0.2")
}

pub fn calculate_twenty_plus_3_payout(side_bet: u32, player_hand: &Hand) -> i64 {
    unimplemented!("Twenty Plus 3 - v0.2")
}

pub fn calculate_royal_match_payout(side_bet: u32, player_hand: &Hand) -> i64 {
    unimplemented!("Royal Match - v0.2")
}

pub fn calculate_lucky_ladies_payout(side_bet: u32, player_hand: &Hand) -> i64 {
    unimplemented!("Lucky Ladies - v0.2")
}

pub fn calculate_bust_it_payout(side_bet: u32, player_hand: &Hand) -> i64 {
    unimplemented!("Bust It - v0.2")
}

pub fn calculate_super_7s_payout(side_bet: u32, player_hand: &Hand) -> i64 {
    unimplemented!("Super 7s - v0.2")
}

pub fn calculate_lucky_lucky_payout(side_bet: u32, player_hand: &Hand) -> i64 {
    unimplemented!("Lucky Lucky - v0.2")
}

pub fn calculate_pair_squared_payout(side_bet: u32, player_hand: &Hand) -> i64 {
    unimplemented!("Pair Squared - v0.2")
}

pub fn calculate_blackjack_spin_payout(side_bet: u32, player_hand: &Hand) -> i64 {
    unimplemented!("Blackjack Spin - v0.2")
}

pub fn calculate_match_the_deal_payout(side_bet: u32, player_hand: &Hand) -> i64 {
    unimplemented!("Match the Deal - v0.2")
}

pub fn calculate_buster_blackjack_payout(side_bet: u32, player_hand: &Hand) -> i64 {
    unimplemented!("Buster Blackjack - v0.2")
}

pub fn calculate_bet_the_set_payout(side_bet: u32, player_hand: &Hand) -> i64 {
    unimplemented!("Bet the Set - v0.2")
}

pub fn calculate_top_3_payout(side_bet: u32, player_hand: &Hand) -> i64 {
    unimplemented!("Top 3 - v0.2")
}

pub fn calculate_hot_3_payout(side_bet: u32, player_hand: &Hand) -> i64 {
    unimplemented!("Hot 3 - v0.2")
}

pub fn calculate_kings_bounty_payout(side_bet: u32, player_hand: &Hand) -> i64 {
    unimplemented!("Kings Bounty - v0.2")
}

// missing progressives: caribbean_21, blazing_7s
