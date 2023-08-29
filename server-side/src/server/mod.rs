use crate::Args;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use game_logic::game::Game;
use game_logic::player::Player;
use game_logic::game::games::Thousand;
use game_logic::writer::PlayerConnector;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::io::{AsyncBufRead, AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt};
use std::sync::{Mutex, Arc};

pub struct Server {
    port: u16,
    server_name: String,
    game: Option<Box<dyn Game>>,
    player_connector: PlayerConnector<String>,
    players: Arc<Mutex<Vec<Player>>>,
}

impl Server {
    pub fn new(args: Args) -> Server {
        let (tx, rx) = broadcast::channel::<String>(20);
        let displayer = PlayerConnector::new(tx, rx);
        Server {
            port: args.port,
            server_name: args.server_name,
            game: None,
            player_connector: displayer,
            players: Arc::new(Mutex::new(vec![])),
        }
    }

    pub async fn run(&self) {
        println!("Starting server");
        let listener = TcpListener::bind(format!("localhost:{}", self.port))
            .await
            .unwrap();
        println!("Binded to port {}", self.port);

        let (tx, _rx) = broadcast::channel::<String>(10);

        loop {
            let (mut socket, addr) = listener.accept()
            .await
            .unwrap();
            println!("New player joinded!");

            let players_arc = Arc::clone(&self.players);

            let (tx, rx) = self.player_connector.clone_tx_rx();
            tokio::spawn(async move {
                let (reader, mut writer) = socket.split();
                let mut buf_reader = BufReader::new(reader);
                let (mut buf_reader, player) = Player::build_tcp(buf_reader).await;
                let player_copy;
                if let Some(player) = player {
                    let mut players_lock = players_arc.lock().unwrap();
                    println!("New player choosed name: {}", player.get_name());
                    player_copy = player.clone();
                    players_lock.push(player);
                } else {
                    writer.write_all("You must choose a name!".as_bytes()).await.unwrap();
                    println!("Unnamed player disconected");
                    return
                }
                let mut buffer = String::new();
                loop {
                    tokio::select! {
                        result = buf_reader.read_line(&mut buffer) => {
                            if result.unwrap() == 0 {
                                println!("Player disconnected: {}", player_copy.get_name());
                                let mut players_lock = players_arc.lock().unwrap();
                                let index = players_lock.iter().position(|x| x.get_name() == player_copy.get_name()).unwrap();
                                players_lock.remove(index);
                                println!("Players remaining: ");
                                for player in players_lock.iter() {
                                    println!("{}", player.get_name());
                                }
                                break;
                            }
                        }
                    }
                }
            });
        }
    }   
}
