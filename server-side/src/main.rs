

use std::{env, fs};
use error::NoSettingsFileError;

mod server;
mod error;

#[tokio::main]
async fn main() {

    let args = env::args();
    let default_settings = fs::read_to_string("./settings.txt")
        .map_err(|_| NoSettingsFileError);

    let default_settings = match default_settings{
        Ok(content) => content,
        Err(err) => {
            eprintln!("{err}");
            return;
        },
    };

    let server = server::Server::new(args, default_settings);
}
