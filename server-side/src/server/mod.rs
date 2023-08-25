use crate::Args;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use game_logic::game::Game;
use game_logic::game::games::Thousand;
use game_logic::writer::{Communicator, BackendConnector};
use tokio::sync::broadcast::{Receiver, Sender};

pub struct Server {
    port: u16,
    server_name: String,
    game: Option<Box<dyn Game>>,
    displayer: Box<dyn Communicator>,
}

impl Server {
    pub fn new(args: Args) -> Server {
        let (rx, tx) = broadcast::channel::<String>(20);
        let displayer = Box::new(BackendConnector::new(tx, rx));
        Server {
            port: args.port,
            server_name: args.server_name,
            game: None,
            displayer: displayer,
        }
    }

    pub async fn run(&self) {
        let listener = TcpListener::bind(format!("localhost{}", self.port))
            .await
            .unwrap();

        let (tx, _rx) = broadcast::channel::<String>(10);

        loop {
            let (mut socket, addr) = listener.accept()
            .await
            .unwrap();

            let tx = tx.clone();
            let mut rx = tx.subscribe();

        }
    }   
}
