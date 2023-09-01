use crate::game::*;
use crate::player::Player;
pub struct Thousand {
    game_name: String,
    max_players: u8,
    min_players: u8,
    players: Vec<Player>,
    rules: ThousandRules,
}

impl Game for Thousand {
    fn start(&self) -> Result<(), GameError> {
        todo!();
    }

    fn stop(&self) {
        todo!()
    }

    fn add_player(&mut self, player: Player) -> Option<()> {
        todo!()
    }


    fn remove_player(&mut self, name_of_player: &str) -> Option<()> {
        todo!()
    }

    fn get_game_name(&self) -> &str {
        &self.game_name
    }

    fn get_min_players(&self) -> u8 {
        self.max_players
    }

    fn get_max_players(&self) -> u8 {
        self.max_players
    }

    fn get_players(&mut self) -> Option<&Vec<Player>> {
        if self.players.is_empty() {
            return None;
        } else {
            return Some(&self.players)
        }
    }
}

impl Thousand {
    pub fn new() -> Thousand {
        Thousand {
            game_name: "Thousand".to_string(),
            min_players: 3,
            max_players: 3,
            players: vec![],
            rules: ThousandRules,
        }
    }
}


pub struct ThousandRules;

