mod card_generator;
pub mod database;
mod tests;

use serde::{Deserialize, Serialize};
use rand::prelude::SliceRandom;
use uuid::Uuid;
use postgres::Client;

#[derive(Serialize)]
pub struct JsonMessage<'a> {
    pub message: &'a str,
}

type Card = i32;

#[derive(Serialize)]
pub struct CardRepresentation {
    pub name: String,
    pub value: Card,
    pub image: String,
}

#[derive(PartialEq, Eq, Deserialize)]
pub enum DeckSize {
    Small,
    Normal,
}

#[derive(Serialize)]
pub struct Deck {
    pub id: Uuid,
    pub cards: Vec<Card>,
    pub discarded: Vec<Card>,
}

impl Deck {
    /// Creates a new card deck with with 52 or 32 cards inside.
    ///
    /// # Arguments
    ///
    /// * `deck_size`: The desired card deck size, 52 for a normal deck either 32 for a small card deck.
    ///
    /// returns: Deck
    ///
    /// # Examples
    ///
    /// ```
    /// use jeu_de_carte::{Deck, DeckSize};
    ///
    /// fn main() {
    ///     let normal_deck = Deck::new(DeckSize::Normal);
    ///     let small_deck = Deck::new(DeckSize::Small);
    /// }
    /// ```
    pub fn new(deck_size: DeckSize) -> Deck {
        let cards = (0..52)
            .filter(|motif| deck_size == DeckSize::Normal || ((motif % 13) < 1 || 5 < (motif % 13)))
            .collect();

        let discarded = Vec::new();

        Deck { id: Uuid::new_v4(), cards, discarded }
    }

    pub fn shuffle(&mut self, shuffle_discarded: bool) {
        let mut rng = rand::thread_rng();
        if shuffle_discarded {
            self.discarded.shuffle(&mut rng);
        } else {
            self.cards.shuffle(&mut rng)
        }
    }

    /// Takes one or more cards of the deck card to put them into the discarded stack.
    ///
    /// # Arguments
    ///
    /// * `lifo`: If true, adds the cards to the top of the deck, otherwise at the end of the deck.
    /// * `length`: Number of cards to take.
    /// * `move_as_block`: If true, moves all cards as one block, otherwise puts cards one by one in the discarded stack.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use jeu_de_carte::{Deck, DeckSize};
    ///
    /// fn main() {
    ///     let mut normal_deck = Deck::new(DeckSize::Normal);
    ///     normal_deck.take(false, 1, true);
    ///     normal_deck.take(false, 5, false);
    /// }
    /// ```
    pub fn take(&mut self, lifo: bool, length: usize, move_as_block: bool) {
        if self.cards.len() == 0 || length == 0 { return; }
        let at = if length >= self.cards.len() {
            0
        } else {
            (self.cards.len() - length) % self.cards.len()
        };

        let mut cards: Vec<Card> = self.cards.drain(at..).collect();
        if lifo {
            if !move_as_block { cards.reverse() }
            self.discarded.extend_from_slice(&cards)
        } else {
            cards.extend_from_slice(&self.discarded);
            self.discarded = cards
        }
    }

    /// Takes one or more cards of the discarded stack to put them into the cards stack.
    ///
    /// # Arguments
    ///
    /// * `lifo`: If true, adds the cards to the top of the deck, otherwise at the end of the deck.
    /// * `length`: Number of cards to take.
    /// * `move_as_block`: If true, moves all cards as one block, otherwise puts cards one by one in the cards stack.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use jeu_de_carte::{Deck, DeckSize};
    ///
    /// fn main() {
    ///     let mut normal_deck = Deck::new(DeckSize::Normal);
    ///     normal_deck.put(false, 1, true);
    ///     normal_deck.put(false, 5, false);
    /// }
    /// ```
    pub fn put(&mut self, lifo: bool, length: usize, move_as_block: bool) {
        if self.discarded.len() == 0 || length == 0 { return; }
        let at = if length >= self.discarded.len() {
            0
        } else {
            (self.discarded.len() - length) % self.discarded.len()
        };

        let mut cards: Vec<Card> = self.discarded.drain(at..).collect();
        if lifo {
            if !move_as_block { cards.reverse() }
            self.cards.extend_from_slice(&cards)
        } else {
            cards.extend_from_slice(&self.cards);
            self.cards = cards
        }
    }

    pub fn insert_into_db(&self, connection: &mut Client) {
        let stmt = connection
            .prepare(
                "INSERT INTO decks (id, cards, discarded) VALUES ($1, $2, $3)"
            ).unwrap();
        connection.execute(&stmt, &[&self.id, &self.cards, &self.discarded]).expect("An error occurred.");
    }

    pub fn update_db(&self, connection: &mut Client) {
        let stmt = connection
            .prepare(
                "UPDATE decks SET cards = $2, discarded = $3 WHERE id = $1"
            ).unwrap();

        connection.execute(&stmt, &[&self.id, &self.cards, &self.discarded]).expect("An error occurred.");
    }

    pub fn delete_from_db(&self, connection: &mut Client) {
        let stmt = connection
            .prepare(
                "DELETE FROM decks WHERE id = $1"
            ).unwrap();

        connection.execute(&stmt, &[&self.id]).expect("An error occurred.");
    }

    pub fn find_by_id(id: Uuid, connection: &mut Client) -> Option<Deck> {
        let stmt = connection
            .prepare(
                "SELECT id, cards, discarded FROM decks WHERE id = $1"
            ).unwrap();
        let rows = connection.query(&stmt, &[&id]);

        match rows {
            Ok(rows) => {
                let row = rows.iter().next();
                match row {
                    Some(row) => {
                        let cards: Vec<Card> = row.get(1);
                        let discarded: Vec<Card> = row.get(2);

                        Some(Deck {
                            id,
                            cards,
                            discarded,
                        })
                    }
                    None => None
                }
            }
            Err(_) => None
        }
    }
}

#[derive(Serialize)]
pub struct DeckRepresentation {
    pub id: Uuid,
    pub cards: Vec<CardRepresentation>,
    pub discarded: Vec<CardRepresentation>,
}

impl DeckRepresentation {
    pub fn from(deck: Deck) -> DeckRepresentation {
        DeckRepresentation {
            id: deck.id,
            cards: DeckRepresentation::translate_cards(deck.cards),
            discarded: DeckRepresentation::translate_cards(deck.discarded),
        }
    }

    fn translate_cards(cards: Vec<Card>) -> Vec<CardRepresentation> {
        cards.iter().map(|motif| card_generator::get_card(motif)).collect()
    }
}
