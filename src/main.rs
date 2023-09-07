use std::{sync::{Mutex, Arc}, io::stdin};
use clap::Parser;
use crate::std_writer::StdWriter;

mod id;
mod player;
mod std_writer;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let writer = Arc::new(StdWriter::new());

    writer.println("Starting program");
   
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 1234)]
    port: u16,
    #[arg(short, long, default_value_t = String::from("d"))]
    server_name: String,
}


