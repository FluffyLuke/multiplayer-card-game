use crate::Args;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use game_logic::game::Game;
use game_logic::player::Player;
use game_logic::game::games::Thousand;
use game_logic::writer::PlayerConnector;
use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};
use std::{sync::{Mutex, Arc}, ops::Index};
use crate::{print_to_stdout, display_current_text, set_text_to_display};

pub struct Server {
    port: u16,
    server_name: String,
    game: Arc<Mutex<Option<Box<dyn Game>>>>,
    player_connector: PlayerConnector<String>,
    players: Arc<Mutex<Vec<Player>>>,
    stdout_writer: Arc<Mutex<StdOutputWriter>>,
    games: Arc<Mutex<Vec<Box<dyn Game + Send>>>>,
}

impl Server {
    pub fn new(args: Args) -> Server {
        let (tx, rx) = broadcast::channel::<String>(20);
        let displayer = PlayerConnector::new(tx, rx);
        let games = vec![Box::new(Thousand::new())];
        Server {
            port: args.port,
            server_name: args.server_name,
            game: Arc::new(Mutex::new(None)),
            player_connector: displayer,
            players: Arc::new(Mutex::new(vec![])),
            stdout_writer: Arc::new(Mutex::new(StdOutputWriter::new())),
            games: Arc::new(Mutex::new(vec![Box::new(Thousand::new())]))
        }
    }

    pub async fn run(&self) {
        println!("Starting server");
        let listener = TcpListener::bind(format!("localhost:{}", self.port))
            .await
            .unwrap();
        println!("Binded to port {}", self.port);

        let (tx, _rx) = broadcast::channel::<PlayerActions>(10);

        //server thread
        let stdout_writer_arc = Arc::clone(&self.stdout_writer);
        let games_arc = Arc::clone(&self.games);
        tokio::spawn(async move {
            let mut input_buffer = String::new();
            let stdin = std::io::stdin();
            let games = games_arc.lock().unwrap();
            loop {
                //Choose game from game list
                let mut games_list = String::new();
                for game in games.iter() {
                    games_list.push_str(&format!("{} ", &*game.get_game_name()));
                }
                set_text_to_display!(stdout_writer_arc, Some(format!("Choose game to play: {}", games_list)));
                display_current_text!(stdout_writer_arc);
                loop {
                    input_buffer.clear();
                    stdin.read_line(&mut input_buffer).unwrap();
                    if input_buffer.ends_with("\n") {
                        input_buffer.pop();
                    }
                    if let Some(game) = games.iter()
                        .find(|&x| {println!("{}", x.get_game_name()); println!("{}", input_buffer); x.get_game_name() == &input_buffer}) {
                        set_text_to_display!(stdout_writer_arc, None);
                        print_to_stdout!(stdout_writer_arc, format!("Game selected: {}", game.get_game_name()));
                        break;
                    } else {
                        print_to_stdout!(stdout_writer_arc, "No game found, try again!");
                    }
                }
                set_text_to_display!(stdout_writer_arc, None);

                
            }
        });

        loop {
            let (mut socket, _addr) = listener.accept()
            .await
            .unwrap();

            let stdout_writer_arc = Arc::clone(&self.stdout_writer);
            print_to_stdout!(stdout_writer_arc, "Player joined!");
            let players_arc = Arc::clone(&self.players);

            let (tx, rx) = self.player_connector.clone_tx_rx();
            tokio::spawn(async move {
                let (reader, mut writer) = socket.split();
                let buf_reader = BufReader::new(reader);
                let (mut buf_reader, player) = Player::build_tcp(buf_reader).await;
                let player_copy;
                if let Some(player) = player {
                    let mut players_lock = players_arc.lock().unwrap();
                    print_to_stdout!(stdout_writer_arc, format!("New player choosed name: {}", player.get_name()));
                    player_copy = player.clone();
                    players_lock.push(player);
                } else {
                    writer.write_all("You must choose a name!".as_bytes()).await.unwrap();
                    print_to_stdout!(stdout_writer_arc, "Player disconnected");
                    return
                }
                let mut buffer = String::new();
                loop {
                    tokio::select! {
                        result = buf_reader.read_line(&mut buffer) => {
                            //player disconnects
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
                            //player plays a card
                        }
                    }
                }
            });
        }
    }   
}

#[derive(Debug, Clone, PartialEq)]
enum PlayerActions {
    
}

struct StdOutputWriter {
    current_text_to_display: Option<String>,
}

impl StdOutputWriter {
    fn new() -> StdOutputWriter {
        StdOutputWriter{ current_text_to_display: None }
    }
    fn print_to_stdout<T: std::fmt::Display>(&self, text: T) {
        println!("{}", text);
        self.display_current_text();
    }
    fn set_text_to_display(&mut self, text: Option<String>) {
        self.current_text_to_display = text;
    }

    fn display_current_text(&self) -> Option<()> {
        if let Some(text) = &self.current_text_to_display {
            println!("{}", text);
            return Some(());
        }
        None
    }
}
#[macro_export]
macro_rules! print_to_stdout {
    ($writer_arc:expr, $text:expr) => {{
        $writer_arc.lock().unwrap().print_to_stdout($text);
    }};
}
#[macro_export]
macro_rules! set_text_to_display {
    ($writer_arc:expr, $text:expr) => {{
        $writer_arc.lock().unwrap().set_text_to_display($text);
    }};
}
#[macro_export]
macro_rules! display_current_text {
    ($writer_arc:expr) => {{
        $writer_arc.lock().unwrap().display_current_text();
    }};
}