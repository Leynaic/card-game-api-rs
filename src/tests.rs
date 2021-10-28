#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use crate::{Deck, DeckSize};

    #[test]
    fn test_create_small_deck() {
        let deck = Deck::new(DeckSize::Small);
        assert_eq!(deck.cards.len(), 32);
    }

    #[test]
    fn test_create_normal_deck() {
        let deck = Deck::new(DeckSize::Normal);
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn test_shuffle_deck() {
        let mut deck = Deck::new(DeckSize::Normal);
        let deck_copy = Deck { id: Uuid::new_v4(), cards: deck.cards.clone(), discarded: vec![] };

        assert_eq!(deck.cards, deck_copy.cards);

        deck.shuffle();

        assert_ne!(deck.cards, deck_copy.cards);
    }
}