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
        let mut aces = 0;

        for card in &self.cards {
            total += card.pip_value();
            if card.rank == Rank::Ace {
                aces += 1;
            }
        }

        while aces > 0 && total + 10 <= 21 {
            total += 10;
            aces -= 1;
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
        let mut total = 0;
        let mut aces = 0;

        for card in &self.cards {
            total += card.pip_value();
            if card.rank == Rank::Ace {
                aces += 1;
            }
        }

        aces > 0 && total + 10 <= 21
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
    fn test_empty_hand() {
        let hand = Hand::new();
        assert_eq!(hand.value(), 0);
        assert!(!hand.is_blackjack());
        assert!(!hand.is_bust());
    }

    #[test]
    fn test_blackjack() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spades));
        hand.add_card(Card::new(Rank::King, Suit::Hearts));
        assert_eq!(hand.value(), 21);
        assert!(hand.is_blackjack());
        assert!(hand.is_soft());
    }

    #[test]
    fn test_soft_hand() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spades));
        hand.add_card(Card::new(Rank::Six, Suit::Hearts));
        assert_eq!(hand.value(), 17); // Ace counted as 11
        assert!(hand.is_soft());
    }

    #[test]
    fn test_hard_hand() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::King, Suit::Spades));
        hand.add_card(Card::new(Rank::Seven, Suit::Hearts));
        assert_eq!(hand.value(), 17);
        assert!(!hand.is_soft());
    }

    #[test]
    fn test_bust() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::King, Suit::Spades));
        hand.add_card(Card::new(Rank::Queen, Suit::Hearts));
        hand.add_card(Card::new(Rank::Five, Suit::Clubs));
        assert_eq!(hand.value(), 25);
        assert!(hand.is_bust());
    }

    #[test]
    fn test_multiple_aces() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spades));
        hand.add_card(Card::new(Rank::Ace, Suit::Hearts));
        hand.add_card(Card::new(Rank::Nine, Suit::Clubs));
        assert_eq!(hand.value(), 21); // 11 + 1 + 9
        assert!(hand.is_soft());
    }

    #[test]
    fn test_soft_becomes_hard() {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spades));
        hand.add_card(Card::new(Rank::Six, Suit::Hearts));
        assert!(hand.is_soft());

        hand.add_card(Card::new(Rank::Ten, Suit::Clubs));
        assert_eq!(hand.value(), 17); // 1 + 6 + 10
        assert!(!hand.is_soft()); // No longer soft
    }
}
