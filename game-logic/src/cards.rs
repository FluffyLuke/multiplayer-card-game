use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Card {
    name: CardType,
    suit: Suit,
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum Suit {
    BlackSpades,
    RedHearts,
    BlueDiamonds,
    GreenClubs,
    Jocker,
}


#[derive(Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum CardType {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Jocker,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CardDeck {
    cards_list: Vec<Card>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hand {
    player: Player,
    player_cards: Vec<Card>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Player {
    Player(String),
    Bot(String),
    None,
}

impl CardDeck {
    pub fn new_basic_deck(number_of_jockers: u8) -> CardDeck {
        let mut card_deck = vec![];
        for suit in Suit::iter() {
            if suit == Suit::Jocker { continue; }
            card_deck.push(Card {name: CardType::Ace, suit});
            card_deck.push(Card {name: CardType::Two, suit});
            card_deck.push(Card {name: CardType::Three, suit});
            card_deck.push(Card {name: CardType::Four, suit});
            card_deck.push(Card {name: CardType::Five, suit});
            card_deck.push(Card {name: CardType::Six, suit});
            card_deck.push(Card {name: CardType::Seven, suit});
            card_deck.push(Card {name: CardType::Eight, suit});
            card_deck.push(Card {name: CardType::Nine, suit});
            card_deck.push(Card {name: CardType::Ten, suit});
            card_deck.push(Card {name: CardType::Jack, suit});
            card_deck.push(Card {name: CardType::Queen, suit});
            card_deck.push(Card {name: CardType::King, suit});
        }
        for _ in 1..=number_of_jockers {
            card_deck.push(Card {name: CardType::Jocker, suit: Suit::Jocker})
        }
        CardDeck {
            cards_list: card_deck,
        } 
    }
    pub fn new(cards: Vec<Card>) -> CardDeck {
        CardDeck {
            cards_list: cards,
        }
    }
}

pub trait Shuffle {
    fn shuffle(&mut self);
}

pub trait SeeCardsInDeck {
    fn see_cards_in_deck(&self) -> &Vec<Card>;
}

pub trait SeeCurrentCards {
    fn see_current_cards(&self) -> &Vec<Card>;
}

#[cfg(test)]
mod tests {

    #[test]
    fn generate_basic_deck() {
        use super::*;
        let deck = CardDeck::new_basic_deck(3);

        let deck_by_hand = {
            let mut card_deck = vec![];
            card_deck.push(Card {name: CardType::Ace, suit: Suit::BlackSpades});
            card_deck.push(Card {name: CardType::Two, suit: Suit::BlackSpades});
            card_deck.push(Card {name: CardType::Three, suit: Suit::BlackSpades});
            card_deck.push(Card {name: CardType::Four, suit: Suit::BlackSpades});
            card_deck.push(Card {name: CardType::Five, suit: Suit::BlackSpades});
            card_deck.push(Card {name: CardType::Six, suit: Suit::BlackSpades});
            card_deck.push(Card {name: CardType::Seven, suit: Suit::BlackSpades});
            card_deck.push(Card {name: CardType::Eight, suit: Suit::BlackSpades});
            card_deck.push(Card {name: CardType::Nine, suit: Suit::BlackSpades});
            card_deck.push(Card {name: CardType::Ten, suit: Suit::BlackSpades});
            card_deck.push(Card {name: CardType::Jack, suit: Suit::BlackSpades});
            card_deck.push(Card {name: CardType::Queen, suit: Suit::BlackSpades});
            card_deck.push(Card {name: CardType::King, suit: Suit::BlackSpades});

            card_deck.push(Card {name: CardType::Ace, suit: Suit::RedHearts});
            card_deck.push(Card {name: CardType::Two, suit: Suit::RedHearts});
            card_deck.push(Card {name: CardType::Three, suit: Suit::RedHearts});
            card_deck.push(Card {name: CardType::Four, suit: Suit::RedHearts});
            card_deck.push(Card {name: CardType::Five, suit: Suit::RedHearts});
            card_deck.push(Card {name: CardType::Six, suit: Suit::RedHearts});
            card_deck.push(Card {name: CardType::Seven, suit: Suit::RedHearts});
            card_deck.push(Card {name: CardType::Eight, suit: Suit::RedHearts});
            card_deck.push(Card {name: CardType::Nine, suit: Suit::RedHearts});
            card_deck.push(Card {name: CardType::Ten, suit: Suit::RedHearts});
            card_deck.push(Card {name: CardType::Jack, suit: Suit::RedHearts});
            card_deck.push(Card {name: CardType::Queen, suit: Suit::RedHearts});
            card_deck.push(Card {name: CardType::King, suit: Suit::RedHearts});

            card_deck.push(Card {name: CardType::Ace, suit: Suit::BlueDiamonds});
            card_deck.push(Card {name: CardType::Two, suit: Suit::BlueDiamonds});
            card_deck.push(Card {name: CardType::Three, suit: Suit::BlueDiamonds});
            card_deck.push(Card {name: CardType::Four, suit: Suit::BlueDiamonds});
            card_deck.push(Card {name: CardType::Five, suit: Suit::BlueDiamonds});
            card_deck.push(Card {name: CardType::Six, suit: Suit::BlueDiamonds});
            card_deck.push(Card {name: CardType::Seven, suit: Suit::BlueDiamonds});
            card_deck.push(Card {name: CardType::Eight, suit: Suit::BlueDiamonds});
            card_deck.push(Card {name: CardType::Nine, suit: Suit::BlueDiamonds});
            card_deck.push(Card {name: CardType::Ten, suit: Suit::BlueDiamonds});
            card_deck.push(Card {name: CardType::Jack, suit: Suit::BlueDiamonds});
            card_deck.push(Card {name: CardType::Queen, suit: Suit::BlueDiamonds});
            card_deck.push(Card {name: CardType::King, suit: Suit::BlueDiamonds});

            card_deck.push(Card {name: CardType::Ace, suit: Suit::GreenClubs});
            card_deck.push(Card {name: CardType::Two, suit: Suit::GreenClubs});
            card_deck.push(Card {name: CardType::Three, suit: Suit::GreenClubs});
            card_deck.push(Card {name: CardType::Four, suit: Suit::GreenClubs});
            card_deck.push(Card {name: CardType::Five, suit: Suit::GreenClubs});
            card_deck.push(Card {name: CardType::Six, suit: Suit::GreenClubs});
            card_deck.push(Card {name: CardType::Seven, suit: Suit::GreenClubs});
            card_deck.push(Card {name: CardType::Eight, suit: Suit::GreenClubs});
            card_deck.push(Card {name: CardType::Nine, suit: Suit::GreenClubs});
            card_deck.push(Card {name: CardType::Ten, suit: Suit::GreenClubs});
            card_deck.push(Card {name: CardType::Jack, suit: Suit::GreenClubs});
            card_deck.push(Card {name: CardType::Queen, suit: Suit::GreenClubs});
            card_deck.push(Card {name: CardType::King, suit: Suit::GreenClubs});

            card_deck.push(Card {name: CardType::Jocker, suit: Suit::Jocker});
            card_deck.push(Card {name: CardType::Jocker, suit: Suit::Jocker});
            card_deck.push(Card {name: CardType::Jocker, suit: Suit::Jocker});
            card_deck
        };

        let deck_by_hand = CardDeck::new(deck_by_hand);

        assert_eq!(deck, deck_by_hand); 
    }
}

