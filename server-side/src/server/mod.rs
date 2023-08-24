use crate::Args;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use game_logic::game::Game;
use game_logic::game::games::Thousand;

pub struct Server {
    port: u16,
    server_name: String,
    game: Box<dyn Game>,
}

impl Server {
    pub fn new(args: Args) -> Server {
        Server {
            port: args.port,
            server_name: args.server_name,
            game: Box::new(Thousand::new()),
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
