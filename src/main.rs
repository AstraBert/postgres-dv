mod dbops;

use crate::dbops::console;
use clap::Parser;

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
    let args = Args::parse();
    console(&args.uri, args.connections).await;
}
