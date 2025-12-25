use richrs::prelude::Console;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

async fn connect(conn_string: &str, max_conns: u32) -> Pool<Postgres> {
    PgPoolOptions::new()
    .max_connections(max_conns).
    connect(conn_string).await.expect("Unable to connect to the specified Postgres database, please check the connection string and the maximum allowed connections and try again.")
}

async fn execute_query(pool: &Pool<Postgres>, query: &str) {
    let rows = sqlx::query(query).fetch_all(pool).await.expect("Unable to execute query, please check the syntax and the connection and try again.");
    for row in rows {
        dbg!(row);
    }
}

pub async fn console(conn_string: &str, max_conns: u32) {
    let mut console = Console::new();
    let pool = connect(conn_string, max_conns).await;
    loop {
        let ans = console.input("[bold purple]Your query:[/] ");
        let answer: String = match ans {
            Ok(s ) => {
                s
            }
            Err(e) => {
                let _ = console.print(&format!("[bold red]An error occurred: {}[/]", e.to_string()));
                return;
            }
        };
        if vec!["q".to_string(), "quit".to_string(), "exit".to_string()].contains(&answer) {
            break;
        } else {
            execute_query(&pool, &answer).await;
        }
    }
}