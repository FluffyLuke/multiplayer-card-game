use rand::thread_rng;
use rand::seq::SliceRandom;
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
    GreenClubs
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

pub struct CardDeck {
    cards_list: Vec<Card>,
    cards_left: Vec<Card>,
}

impl CardDeck {
    pub fn new_basic(number_of_jockers: u8) -> CardDeck {
        //TODO END THIS METHOD
        let mut card_vec = vec![];
        for suit in Suit::iter() {
            for name in CardType::iter() {
                card_vec.push(
                    Card {
                        name,
                        suit,
                    }
                );
            }
        }
        card_vec.retain(|&x| x.name == CardType::Jocker);

    }
    pub fn new(cards: Vec<Card>) -> CardDeck {
        CardDeck {
            cards_list: cards.clone(),
            cards_left: cards,
        }
    }
    pub fn shuffle(&mut self) {
        self.cards_list.shuffle(&mut thread_rng());
    }
    pub fn see_cards_currently_in_deck(&self) -> &Vec<Card>{
        &self.cards_left
    }
    pub fn see_all_possible_cards(&self) -> &Vec<Card>{
        &self.cards_list
    }
}