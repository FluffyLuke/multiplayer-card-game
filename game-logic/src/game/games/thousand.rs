use crate::game::*;
use crate::player::Player;
pub struct Thousand {
    game_name: String,
    max_players: u8,
    min_players: u8,
    players: Vec<Player>,
    rules: ThousandRules,
}

impl Thousand {
    pub fn new(players: Vec<Player>) -> Thousand {
        Thousand {
            game_name: "Thousand".to_string(),
            min_players: 3,
            max_players: 3,
            players: players,
            rules: ThousandRules,
        }
    }
}
pub struct ThousandRules;

