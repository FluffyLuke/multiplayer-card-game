#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Card {
    name: CardType,
    color: Suit,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
    BlackSpades,
    RedHearts,
    BlueDiamonds,
    GreenClubs
}


#[derive(Debug, Copy, Clone, PartialEq)]pub enum CardType {
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