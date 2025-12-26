mod dbops;
mod validate;

use crate::dbops::console;
use crate::validate::is_valid_postgres_connection_string;
use clap::Parser;
use richrs::prelude::{Console, Markdown};

const WELCOME_MESSAGE: &str = r#"
## postgres-dv

Welcome to **Postgres Data Viewer**, a simple-yet-powerful console for visualizing data from **remote and local** Postgres databases.

Here are a few tips:

- You can only use `SELECT` queries, that respect the syntax: `SELECT some_column FROM some_table ...other clauses...;`
- You can exit the console by typing `e`, `exit`, `q` or `quit`
- You can clear the console by typing `c` or `clear`
"#;

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
        println!();
        let md = Markdown::new(WELCOME_MESSAGE);
        termui
            .write_segments(&md.render(100))
            .expect("Should be able to print welcome message");
        println!();
        console(&args.uri, args.connections).await;
    } else {
        let _ = termui.print("[bold red]ERROR: the connection string you provided is not valid.[/]\nPlease retry with a new one");
    }
}
