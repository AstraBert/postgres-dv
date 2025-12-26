mod dbops;
mod validate;

use crate::dbops::console;
use crate::validate::{is_valid_postgres_connection_string};
use clap::Parser;
use richrs::prelude::Console;

/// Postgres data viewer, a simple console
/// to have a rich view of data from a 
/// remote or local Postgres database
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Postgres database URI
    #[arg(short, long)]
    uri: String,

    /// Number of connections in the pool
    #[arg(short, long, default_value_t = 1)]
    connections: u32,
}

#[tokio::main]
async fn main() {
    let mut termui = Console::new();
    let args = Args::parse();
    if is_valid_postgres_connection_string(&args.uri) {
        console(&args.uri, args.connections).await;
    } else {
        let _ = termui.print("[bold red]ERROR: the connection string you provided is not valid.[/]\nPlease retry with a new one");
    }
}
