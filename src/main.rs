mod dbops;
mod validate;

use crate::dbops::console;
use crate::validate::is_valid_postgres_connection_string;
use clap::{Parser, command};
use richrs::{
    prelude::{Console, Markdown},
    style::Style,
};
use rpassword::prompt_password;

const WELCOME_MESSAGE: &str = r#"
## postgres-dv

Welcome to **Postgres Data Viewer**, a simple-yet-powerful console for visualizing data from **remote and local** Postgres databases.

Here are a few tips:

- You can only use `SELECT` queries, that respect the syntax: `SELECT some_column FROM some_table ...other clauses...;`
- You can exit the console by typing `e`, `exit`, `q` or `quit`
- You can clear the console by typing `c` or `clear`
"#;

const LOGO: &str = r#"
                     _____                                  _________       
_______________________  /_______ ____________________      ______  /__   __
___  __ \  __ \_  ___/  __/_  __ `/_  ___/  _ \_  ___/_______  __  /__ | / /
__  /_/ / /_/ /(__  )/ /_ _  /_/ /_  /   /  __/(__  )_/_____/ /_/ / __ |/ / 
_  .___/\____//____/ \__/ _\__, / /_/    \___//____/        \__,_/  _____/  
/_/                       /____/                                            
"#;

/// Postgres data viewer, a simple console
/// to have a rich view of data from a
/// remote or local Postgres database
#[derive(Parser, Debug)]
#[command(version = "0.1.0")]
#[command(name = "postgres-dv")]
#[command(about, long_about = None)]
struct Args {
    /// Postgres database URI (prefer using the `--stdin` flag for more security)
    #[arg(short, long, default_value = "")]
    uri: String,

    /// Number of connections in the pool
    #[arg(short, long, default_value_t = 1)]
    connections: u32,

    /// Whether or not to pass the connection string from stdin (safer).
    #[arg(short, long, default_value_t = false)]
    stdin: bool,
}

#[tokio::main]
async fn main() {
    let mut termui = Console::new();
    let args = Args::parse();
    let connection_string: String;
    if !args.uri.is_empty() {
        connection_string = args.uri;
    } else if args.stdin {
        connection_string = match prompt_password("You Postgres connection string: ") {
            Ok(s) => s.trim().to_string(),
            Err(e) => {
                let _ = termui.print(&format!("[bold red]ERROR: {}", e));
                return;
            }
        };
    } else {
        let _ = termui.print("[bold red]You should provide one between --uri and --stdin[/]");
        return;
    }
    if is_valid_postgres_connection_string(&connection_string) {
        let _ = termui.print_styled(
            LOGO,
            &Style::new().with_color(richrs::color::Color::Rgb {
                r: 245,
                g: 152,
                b: 39,
            }),
        );
        println!();
        let md = Markdown::new(WELCOME_MESSAGE);
        termui
            .write_segments(&md.render(100))
            .expect("Should be able to print welcome message");
        println!();
        console(&connection_string, args.connections).await;
    } else {
        let _ = termui.print("[bold red]ERROR: the connection string you provided is not valid.[/]\nPlease retry with a new one");
    }
}
