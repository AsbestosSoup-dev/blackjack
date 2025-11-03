use crate::core::card::{Card, Rank};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Hand { cards: Vec::new() }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn value(&self) -> u8 {
        let mut total = 0;
        let mut has_aces = false;

        for card in &self.cards {
            total += card.pip_value();
            if card.rank == Rank::Ace {
                has_aces = true;
            }
        }

        if has_aces && total + 10 <= 21 {
            total += 10;
        }

        total
    }

    pub fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.value() == 21
    }

    pub fn is_bust(&self) -> bool {
        self.value() > 21
    }

    pub fn is_soft(&self) -> bool {
        let hand_value = self.value();
        if hand_value > 21 {
            return false;
        }

        let mut hard_total = 0u8;
        let mut has_aces = false;

        for card in &self.cards {
            hard_total += card.pip_value();
            if card.rank == Rank::Ace {
                has_aces = true;
            }
        }

        has_aces && hand_value != hard_total
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::card::Suit;

    #[test]
    fn empty_hand() {
        let hand = Hand::new();
        assert_eq!(hand.value(), 0);
        assert!(!hand.is_blackjack());
        assert!(!hand.is_bust());
        assert!(!hand.is_soft());
    }

    #[test]
    fn blackjack() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spades));
        hand.add_card(Card::new(Rank::King, Suit::Hearts));
        assert_eq!(hand.value(), 21);
        assert!(hand.is_blackjack());
        assert!(hand.is_soft());
    }

    #[test]
    fn soft_17() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spades));
        hand.add_card(Card::new(Rank::Six, Suit::Hearts));
        assert_eq!(hand.value(), 17);
        assert!(hand.is_soft());
        assert!(!hand.is_blackjack());
    }

    #[test]
    fn hard_17() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::King, Suit::Spades));
        hand.add_card(Card::new(Rank::Seven, Suit::Hearts));
        assert_eq!(hand.value(), 17);
        assert!(!hand.is_soft());
    }

    #[test]
    fn bust() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::King, Suit::Spades));
        hand.add_card(Card::new(Rank::Queen, Suit::Hearts));
        hand.add_card(Card::new(Rank::Five, Suit::Clubs));
        assert_eq!(hand.value(), 25);
        assert!(hand.is_bust());
        assert!(!hand.is_soft());
    }

    #[test]
    fn multiple_aces() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spades));
        hand.add_card(Card::new(Rank::Ace, Suit::Hearts));
        hand.add_card(Card::new(Rank::Nine, Suit::Clubs));
        assert_eq!(hand.value(), 21);
        assert!(hand.is_soft());
        assert!(!hand.is_blackjack());
    }

    #[test]
    fn soft_becomes_hard() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spades));
        hand.add_card(Card::new(Rank::Six, Suit::Hearts));
        assert!(hand.is_soft());

        hand.add_card(Card::new(Rank::Ten, Suit::Clubs));
        assert_eq!(hand.value(), 17);
        assert!(!hand.is_soft());
    }

    #[test]
    fn three_aces() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spades));
        hand.add_card(Card::new(Rank::Ace, Suit::Hearts));
        hand.add_card(Card::new(Rank::Ace, Suit::Clubs));
        assert_eq!(hand.value(), 13);
        assert!(hand.is_soft());
    }
}
