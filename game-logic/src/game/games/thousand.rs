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

impl Game for Thousand {
    fn stop(&self){}
    fn get_max_players(&self) -> u8 { self.max_players }
    fn get_min_players(&self) -> u8 { self.min_players }
    fn get_game_name(&self) -> &str { &self.game_name }
    fn add_player(&mut self, player: Player) -> Option<()> {
        if self.players.iter().count() == self.max_players.into() {
            return None
        };
        self.players.push(player);
        return Some(());
    }
    fn remove_player(&mut self, name_of_player: &str) -> Option<()> { 
        let if_found = self.players.iter()
            .find(|&x| x.get_name() == name_of_player);
        if let Some(_) = if_found {
            return Some(());
        } else {
            return None;
        }
    }
    fn list_players(&mut self) -> Option<&Vec<Player>> {
        if self.players.is_empty() {
            return None
        } else {
            return Some(&self.players);
        }
    }
    fn start(&self) -> Result<(), GameError> {
        todo!("Not yet implementent");
    }
}

pub struct ThousandRules;

impl Rules for Thousand {

}