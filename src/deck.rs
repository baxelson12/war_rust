use crate::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Deck {
    pub deck: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut next_deck = Vec::new();
        for suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in &[
                Rank::Ace,
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
            ] {
                next_deck.push(Card::new(suit.clone(), rank.clone()));
            }
        }
        return Deck { deck: next_deck };
    }

    pub fn shuffle(&mut self) {
        self.deck.shuffle(&mut rand::thread_rng());
    }

    pub fn draw(&mut self) -> Option<Card> {
        return Some(self.deck.remove(0));
    }

    pub fn insert(&mut self, cards: Vec<Card>) {
        self.deck.extend(cards);
    }

    pub fn is_empty(&self) -> bool {
        return self.deck.is_empty();
    }
}
