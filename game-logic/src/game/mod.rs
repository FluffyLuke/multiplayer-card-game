use crate::player::Player;
use std::fmt;

pub mod games;
pub trait Game {
    fn start(&self) -> Result<(), GameError>;

    fn stop(&self);

    fn get_max_players(&self) -> u8;

    fn get_min_players(&self) -> u8;

    fn get_game_name(&self) -> &str;

    fn add_player(&mut self, player: Player) -> Option<()>;

    fn remove_player(&mut self, name_of_player: &str) -> Option<()>;

    fn list_players(&mut self) -> Option<&Vec<Player>>;
}

pub struct BasicGame;

impl Game for BasicGame {
    fn start(&self) -> Result<(), GameError> { Err(GameError::NoGameChoosen) }
    fn stop(&self){}
    fn get_max_players(&self) -> u8 {0}
    fn get_min_players(&self) -> u8 {0}
    fn get_game_name(&self) -> &str {"PlaceHolderGame"}
    fn add_player(&mut self, player: Player) -> Option<()> {None}
    fn remove_player(&mut self, name_of_player: &str) -> Option<()> {None}
    fn list_players(&mut self) -> Option<&Vec<Player>> {None}
}

pub trait Rules {

}

//Errors

#[derive(Debug, Clone)]
pub enum GameError {
    NotEnoughPlayers(u8, u8),
    NoGameChoosen
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::NotEnoughPlayers(min_n_of_players, current_n_of_players) => 
                write!(f, "Not enough players to start the game, minimum is {min_n_of_players}, while now there are {current_n_of_players}."),
            GameError::NoGameChoosen => 
                write!(f, "No game choosen."),
        }
    }
}