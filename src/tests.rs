#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use crate::{Card, Deck, DeckSize};

    #[test]
    fn test_create_small_deck() {
        let deck = Deck::new(DeckSize::Small);
        assert_eq!(deck.cards.len(), 32);
        assert_eq!(deck.discarded.len(), 0);
    }

    #[test]
    fn test_create_normal_deck() {
        let deck = Deck::new(DeckSize::Normal);
        assert_eq!(deck.cards.len(), 52);
        assert_eq!(deck.discarded.len(), 0);
    }

    #[test]
    fn test_shuffle_deck() {
        let mut deck = Deck::new(DeckSize::Normal);
        let deck_copy = Deck { id: Uuid::new_v4(), cards: deck.cards.clone(), discarded: vec![] };

        assert_eq!(deck.cards, deck_copy.cards);
        assert_eq!(deck.discarded, deck_copy.discarded);

        deck.shuffle(false);

        assert_ne!(deck.cards, deck_copy.cards);
        assert_eq!(deck.discarded, deck_copy.discarded);
    }

    #[test]
    fn test_shuffle_discarded_deck() {
        let mut deck = Deck::new(DeckSize::Normal);
        deck.take(false, 52, true);
        let deck_copy = Deck { id: Uuid::new_v4(), cards: vec![], discarded: deck.discarded.clone() };

        assert_eq!(deck.cards, deck_copy.cards);
        assert_eq!(deck.discarded, deck_copy.discarded);

        deck.shuffle(true);

        assert_ne!(deck.discarded, deck_copy.discarded);
        assert_eq!(deck.cards, deck_copy.cards);
    }

    #[test]
    fn test_take_lifo_deck() {
        // Test without move as block
        let mut witness_deck = Deck::new(DeckSize::Normal);
        let mut deck = Deck::new(DeckSize::Normal);

        deck.take(true, 1, false);
        assert_eq!(deck.discarded.first(), witness_deck.cards.last());

        deck.take(true, 9, false);
        let mut witness_cards : Vec<Card> = witness_deck.cards.clone().drain(42..).collect();
        witness_cards.reverse();
        assert_eq!(deck.discarded, witness_cards);

        // Test with move as block
        witness_deck = Deck::new(DeckSize::Normal);
        deck = Deck::new(DeckSize::Normal);

        deck.take(true, 10, true);
        witness_cards = witness_deck.cards.clone().drain(42..).collect();
        assert_eq!(deck.discarded, witness_cards);
    }

    #[test]
    fn test_take_fifo_deck() {
        // Test without move as block
        let mut witness_deck = Deck::new(DeckSize::Normal);
        let mut deck = Deck::new(DeckSize::Normal);

        deck.take(false, 1, false);
        assert_eq!(deck.discarded.first(), witness_deck.cards.last());

        deck.take(false, 9, false);
        let mut witness_cards : Vec<Card> = witness_deck.cards.clone().drain(42..).collect();
        assert_eq!(deck.discarded, witness_cards);


        // Test with move as block
        witness_deck = Deck::new(DeckSize::Normal);
        deck = Deck::new(DeckSize::Normal);

        deck.take(false, 10, true);
        witness_cards = witness_deck.cards.clone().drain(42..).collect();
        assert_eq!(deck.discarded, witness_cards);

        deck.take(false, 10, true);
        witness_cards = witness_deck.cards.clone().drain(32..).collect();
        assert_eq!(deck.discarded, witness_cards);
    }

    #[test]
    fn test_put_lifo_deck() {
        // Test without move as block
        let mut witness_deck = Deck::new(DeckSize::Normal);
        let mut deck = Deck::new(DeckSize::Normal);
        deck.take(false, 52, false);
        witness_deck.take(false, 52, false);
        assert_eq!(deck.discarded.len(), 52);
        assert_eq!(witness_deck.discarded.len(), 52);


        deck.put(true, 1, false);
        assert_eq!(deck.cards.first(), witness_deck.discarded.last());

        deck.put(true, 6, false);
        let mut witness_cards : Vec<Card> = witness_deck.discarded.clone().drain(45..).collect();
        witness_cards.reverse();
        assert_eq!(deck.cards, witness_cards);

        // Test with move as block
        deck.take(true, 7, false);
        assert_eq!(deck.discarded.len(), 52);
        assert_eq!(witness_deck.discarded.len(), 52);

        deck.put(true, 3, true);
        witness_cards = witness_deck.discarded.clone().drain(49..).collect();
        assert_eq!(deck.cards, witness_cards);
    }

    #[test]
    fn test_put_fifo_deck() {
        // Test without move as block
        let mut witness_deck = Deck::new(DeckSize::Normal);
        let mut deck = Deck::new(DeckSize::Normal);
        deck.take(false, 52, false);
        witness_deck.take(false, 52, false);
        assert_eq!(deck.discarded.len(), 52);
        assert_eq!(witness_deck.discarded.len(), 52);


        deck.put(false, 1, false);
        assert_eq!(deck.cards.first(), witness_deck.discarded.last());

        deck.put(false, 4, false);
        let mut witness_cards : Vec<Card> = witness_deck.discarded.clone().drain(47..).collect();
        assert_eq!(deck.cards, witness_cards);

        // Test with move as block
        witness_deck = Deck::new(DeckSize::Normal);
        deck = Deck::new(DeckSize::Normal);
        deck.take(false, 52, false);
        witness_deck.take(false, 52, false);
        assert_eq!(deck.discarded.len(), 52);
        assert_eq!(witness_deck.discarded.len(), 52);

        deck.put(false, 1, true);
        witness_cards = witness_deck.discarded.clone().drain(51..).collect();
        assert_eq!(deck.cards, witness_cards);

        deck.put(false, 4, true);
        witness_cards = witness_deck.discarded.clone().drain(47..).collect();
        assert_eq!(deck.cards, witness_cards);
    }
}