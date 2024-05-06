mod card;
mod deck;

use std::thread;
use std::time::Duration;

use crate::card::Card;
use crate::deck::Deck;

fn compare_cards(card_one: &Card, card_two: &Card) -> usize {
    match card_one.rank.cmp(&card_two.rank) {
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => 2,
    }
}

fn battle(deck_one: &mut Deck, deck_two: &mut Deck) -> usize {
    let card_one = deck_one.draw().unwrap();
    let card_two = deck_two.draw().unwrap();
    let result = compare_cards(&card_one, &card_two);

    println!("{:?} vs {:?}", &card_one, &card_two);
    match result {
        0 => {
            // Not enough cards..
            if deck_one.deck.len() < 2 || deck_two.deck.len() < 2 {
                return 0;
            }
            println!("War..");
            let winnings = vec![
                card_one,
                card_two,
                deck_one.draw().unwrap(),
                deck_two.draw().unwrap(),
            ];
            let result = battle(deck_one, deck_two);

            match result {
                0 => return 0,
                1 => {
                    deck_one.insert(winnings);
                    return 1;
                }
                2 => {
                    deck_two.insert(winnings);
                    return 2;
                }
                _ => unreachable!("Should only receive 0-2 in war."),
            }
        }
        1 => {
            deck_one.insert(vec![card_one, card_two]);
            return 1;
        }
        2 => {
            deck_two.insert(vec![card_one, card_two]);
            return 2;
        }
        _ => unreachable!("Obtained unexpected result comparing cards."),
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    let half_len = deck.deck.len() / 2;
    let mut deck_one = Deck {
        deck: deck
            .deck
            .get(..half_len)
            .expect("Deck one ran out of cards")
            .into(),
    };

    let mut deck_two = Deck {
        deck: deck
            .deck
            .get(half_len..)
            .expect("Deck two ran out of cards")
            .into(),
    };

    while !deck_one.is_empty() && !deck_two.is_empty() {
        let result = battle(&mut deck_one, &mut deck_two);
        // Account for scenario where war occurs, but one player does not have enough cards to
        // continue war.
        match result {
            0 => break,
            1 => println!("Deck one wins"),
            2 => println!("Deck two wins"),
            _ => unreachable!("Should only receive a value of 0-2"),
        }
        thread::sleep(Duration::from_millis(50));
    }

    match deck_one.deck.len() > deck_two.deck.len() {
        true => println!("Deck one wins."),
        false => println!("Deck two wins."),
    }
    println!("Deck one has {} cards remaining", deck_one.deck.len());
    println!("Deck two has {} cards remaining", deck_two.deck.len());
}
