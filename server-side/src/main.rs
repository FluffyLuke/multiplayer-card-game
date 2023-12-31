use clap::Parser;
mod error;
mod server;


#[tokio::main]
async fn main() {
    let args = Args::parse();

    let server = server::Server::new(args);

    server.run().await;
}
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 1234)]
    port: u16,
    #[arg(short, long)]
    server_name: String,
}

