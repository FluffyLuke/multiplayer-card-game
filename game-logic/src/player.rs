use std::io::Read;

use rand::seq::SliceRandom;
use rand::thread_rng;
use tokio::io::AsyncBufReadExt;
use tokio::net::tcp::ReadHalf;

use crate::cards::AddCard;
use crate::cards::CannotPlayCardError;
use crate::cards::Card;
use crate::cards::PlayCard;
use crate::cards::SeeCurrentCards;
use crate::cards::Shuffle;
use tokio::io::{AsyncBufRead, AsyncReadExt, AsyncWriteExt, BufReader};

impl Shuffle for Player {
    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng())
    }
}

impl SeeCurrentCards for Player {
    fn see_current_cards(&self) -> &Vec<Card> {
        &self.cards
    }
}

impl AddCard for Player {
    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}

impl PlayCard for Player {
    fn play_card(&mut self, card: &Card) -> Result<Card, CannotPlayCardError> {

        if self.cards.contains(card) {
            let index = self.cards.iter().position(|x| x == card).unwrap();
            let pushed_card = self.cards.remove(index);
            return Ok(pushed_card);
        };

        Err(CannotPlayCardError)
    }
}

impl Player {
    pub fn new(name: String, cards: Vec<Card>) -> Player {
        Player {
            cards,
            name,
        }
    }

    pub async fn build_tcp(mut buf_reader: BufReader<ReadHalf<'_>>) -> (BufReader<ReadHalf<'_>>, Option<Player>){
        let name;
        let mut buffer = String::new();
        let result = buf_reader.read_line(&mut buffer).await.ok();
        if let None = result {
            return (buf_reader, None)
        }
        if let Some(sent_name) = buffer.strip_prefix("name=") {
            name = sent_name;
        } else {
            return (buf_reader, None)
        }

        (buf_reader, Some(Player {
            name: name.to_string(),
            cards: vec![]
        }))
    }

    pub fn change_cards(&mut self, new_cards: Vec<Card>) {
        self.cards = new_cards;
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    cards: Vec<Card>,
    name: String,
}
