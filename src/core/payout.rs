use crate::core::hand::Hand;
use crate::core::rules::Rules;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum BlackjackPayout {
    Standard, // 3:2
    Vegas,    // 6:5
    Custom { numerator: u32, denominator: u32 },
}

pub fn calculate_payout(bet: u32, player_hand: &Hand, dealer_hand: &Hand, rules: &Rules) -> u64 {
    if player_hand.is_bust() {
        return 0;
    }

    if dealer_hand.is_bust() {
        return bet as u64 * 2;
    }

    let player_hand_value = player_hand.value();
    let dealer_hand_value = dealer_hand.value();

    match player_hand_value.cmp(&dealer_hand_value) {
        std::cmp::Ordering::Greater => {
            if player_hand.is_blackjack() {
                bet as u64 + calculate_payout_for_blackjack(bet, rules.blackjack_payout)
            } else {
                bet as u64 * 2
            }
        }
        std::cmp::Ordering::Less => 0,
        std::cmp::Ordering::Equal => bet as u64,
    }
}

fn calculate_payout_for_blackjack(bet: u32, blackjack_payout: BlackjackPayout) -> u64 {
    match blackjack_payout {
        BlackjackPayout::Standard => (bet * 3 / 2) as u64,
        BlackjackPayout::Vegas => (bet * 6 / 5) as u64,
        BlackjackPayout::Custom {
            numerator,
            denominator,
        } => (bet * numerator / denominator) as u64,
    }
}

pub fn calculate_insurance_payout(insurance_bet: u32, dealer_hand: &Hand) -> u64 {
    if dealer_hand.is_blackjack() {
        insurance_bet as u64 * 2
    } else {
        0
    }
}

pub fn calculate_perfect_pairs_payout(_side_bet: u32, _player_hand: &Hand) -> u64 {
    unimplemented!("Perfect Pairs - v0.2")
}

pub fn calculate_twenty_plus_3_payout(_side_bet: u32, _player_hand: &Hand) -> u64 {
    unimplemented!("Twenty Plus 3 - v0.2")
}

pub fn calculate_royal_match_payout(_side_bet: u32, _player_hand: &Hand) -> u64 {
    unimplemented!("Royal Match - v0.2")
}

pub fn calculate_lucky_ladies_payout(_side_bet: u32, _player_hand: &Hand) -> u64 {
    unimplemented!("Lucky Ladies - v0.2")
}

pub fn calculate_bust_it_payout(_side_bet: u32, _player_hand: &Hand) -> u64 {
    unimplemented!("Bust It - v0.2")
}

pub fn calculate_super_7s_payout(_side_bet: u32, _player_hand: &Hand) -> u64 {
    unimplemented!("Super 7s - v0.2")
}

pub fn calculate_lucky_lucky_payout(_side_bet: u32, _player_hand: &Hand) -> u64 {
    unimplemented!("Lucky Lucky - v0.2")
}

pub fn calculate_pair_squared_payout(_side_bet: u32, _player_hand: &Hand) -> u64 {
    unimplemented!("Pair Squared - v0.2")
}

pub fn calculate_blackjack_spin_payout(_side_bet: u32, _player_hand: &Hand) -> u64 {
    unimplemented!("Blackjack Spin - v0.2")
}

pub fn calculate_match_the_deal_payout(_side_bet: u32, _player_hand: &Hand) -> u64 {
    unimplemented!("Match the Deal - v0.2")
}

pub fn calculate_buster_blackjack_payout(_side_bet: u32, _player_hand: &Hand) -> u64 {
    unimplemented!("Buster Blackjack - v0.2")
}

pub fn calculate_bet_the_set_payout(_side_bet: u32, _player_hand: &Hand) -> u64 {
    unimplemented!("Bet the Set - v0.2")
}

pub fn calculate_top_3_payout(_side_bet: u32, _player_hand: &Hand) -> u64 {
    unimplemented!("Top 3 - v0.2")
}

pub fn calculate_hot_3_payout(_side_bet: u32, _player_hand: &Hand) -> u64 {
    unimplemented!("Hot 3 - v0.2")
}

pub fn calculate_kings_bounty_payout(_side_bet: u32, _player_hand: &Hand) -> u64 {
    unimplemented!("Kings Bounty - v0.2")
}

// missing progressives: caribbean_21, blazing_7s

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::card::{Card, Rank, Suit};
    use crate::core::hand::Hand;

    fn make_rules() -> Rules {
        Rules {
            min_bet: 10,
            max_bet: 500,
            starting_credits: 1000,
            blackjack_payout: BlackjackPayout::Standard,
            num_decks: 6,
            split_limit: 3,
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

    fn make_hand(cards: &[(Rank, Suit)]) -> Hand {
        let mut hand = Hand::new();
        for &(rank, suit) in cards {
            hand.add_card(Card::new(rank, suit));
        }
        hand
    }

    #[test]
    fn player_bust() {
        let rules = make_rules();
        let player = make_hand(&[
            (Rank::King, Suit::Spades),
            (Rank::Queen, Suit::Hearts),
            (Rank::Five, Suit::Clubs),
        ]);
        let dealer = make_hand(&[(Rank::Ten, Suit::Spades), (Rank::Seven, Suit::Hearts)]);

        assert_eq!(calculate_payout(100, &player, &dealer, &rules), 0);
    }

    #[test]
    fn dealer_bust() {
        let rules = make_rules();
        let player = make_hand(&[(Rank::Ten, Suit::Spades), (Rank::Seven, Suit::Hearts)]);
        let dealer = make_hand(&[
            (Rank::King, Suit::Spades),
            (Rank::Queen, Suit::Hearts),
            (Rank::Five, Suit::Clubs),
        ]);

        assert_eq!(calculate_payout(100, &player, &dealer, &rules), 200);
    }

    #[test]
    fn player_wins() {
        let rules = make_rules();
        let player = make_hand(&[(Rank::Ten, Suit::Spades), (Rank::Nine, Suit::Hearts)]);
        let dealer = make_hand(&[(Rank::Ten, Suit::Spades), (Rank::Seven, Suit::Hearts)]);

        assert_eq!(calculate_payout(100, &player, &dealer, &rules), 200);
    }

    #[test]
    fn dealer_wins() {
        let rules = make_rules();
        let player = make_hand(&[(Rank::Ten, Suit::Spades), (Rank::Seven, Suit::Hearts)]);
        let dealer = make_hand(&[(Rank::Ten, Suit::Spades), (Rank::Nine, Suit::Hearts)]);

        assert_eq!(calculate_payout(100, &player, &dealer, &rules), 0);
    }

    #[test]
    fn push() {
        let rules = make_rules();
        let player = make_hand(&[(Rank::Ten, Suit::Spades), (Rank::Seven, Suit::Hearts)]);
        let dealer = make_hand(&[(Rank::King, Suit::Spades), (Rank::Seven, Suit::Hearts)]);

        assert_eq!(calculate_payout(100, &player, &dealer, &rules), 100);
    }

    #[test]
    fn player_blackjack_wins() {
        let rules = make_rules();
        let player = make_hand(&[(Rank::Ace, Suit::Spades), (Rank::King, Suit::Hearts)]);
        let dealer = make_hand(&[(Rank::Ten, Suit::Spades), (Rank::Nine, Suit::Hearts)]);

        assert_eq!(calculate_payout(100, &player, &dealer, &rules), 250);
    }

    #[test]
    fn both_blackjack_push() {
        let rules = make_rules();
        let player = make_hand(&[(Rank::Ace, Suit::Spades), (Rank::King, Suit::Hearts)]);
        let dealer = make_hand(&[(Rank::Ace, Suit::Hearts), (Rank::Queen, Suit::Spades)]);

        assert_eq!(calculate_payout(100, &player, &dealer, &rules), 100);
    }

    #[test]
    fn blackjack_vegas_payout() {
        let mut rules = make_rules();
        rules.blackjack_payout = BlackjackPayout::Vegas;

        let player = make_hand(&[(Rank::Ace, Suit::Spades), (Rank::King, Suit::Hearts)]);
        let dealer = make_hand(&[(Rank::Ten, Suit::Spades), (Rank::Nine, Suit::Hearts)]);

        assert_eq!(calculate_payout(100, &player, &dealer, &rules), 220);
    }

    #[test]
    fn blackjack_custom_2_to_1() {
        let mut rules = make_rules();
        rules.blackjack_payout = BlackjackPayout::Custom {
            numerator: 2,
            denominator: 1,
        };

        let player = make_hand(&[(Rank::Ace, Suit::Spades), (Rank::King, Suit::Hearts)]);
        let dealer = make_hand(&[(Rank::Ten, Suit::Spades), (Rank::Nine, Suit::Hearts)]);

        assert_eq!(calculate_payout(100, &player, &dealer, &rules), 300);
    }
}
