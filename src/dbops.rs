use richrs::prelude::{Console, Table, Column};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use sqlx_pgrow_serde::{read_header, read_row};
use crate::validate::is_valid_select_query;

async fn connect(conn_string: &str, max_conns: u32) -> Pool<Postgres> {
    PgPoolOptions::new()
    .max_connections(max_conns).
    connect(conn_string).await.expect("Unable to connect to the specified Postgres database, please check the connection string and the maximum allowed connections and try again.")
}

async fn execute_query(pool: &Pool<Postgres>, query: &str, console: &mut Console) {
    let rows = sqlx::query(query).fetch_all(pool).await.expect("Unable to execute query, please check the syntax and the connection and try again.");
    if rows.len() > 0 {
        let mut table = Table::new();
        let headers = read_header(&rows[0]);
        for header in headers {
            table.add_column(Column::new(header));
        }
        for row in rows {
            let mut row_vals: Vec<String> = vec![];
            let row_values = read_row(&row);
            for value in row_values {
                row_vals.push(value.to_string());
            }
            table.add_row_cells(row_vals);
        }
        console.write_segments(&table.render(100000)).expect("Console should be able to render the table");
    }
}

pub async fn console(conn_string: &str, max_conns: u32) {
    let mut console = Console::new();
    let pool = connect(conn_string, max_conns).await;
    loop {
        let answer = console.input("[bold cyan]Your query:[/] ").expect("Should be able to take input from console");
        if vec!["q".to_string(), "quit".to_string(), "exit".to_string()].contains(&answer) {
            break;
        } else {
            if is_valid_select_query(&answer) {
                if answer.contains("SELECT *") {
                    let proceed = console.input("[bold yellow]Are you sure you want to select all columns from the table?[/] (yes/no) ").expect("You should be able to confirm");
                    if vec!["yes".to_string(), "y".to_string(), "yse".to_string()].contains(&proceed.to_lowercase()) {
                        execute_query(&pool, &answer, &mut console).await;
                    } else {
                        continue;
                    }
                } else {
                    execute_query(&pool, &answer, &mut console).await;
                }
            } else {
                let _ = console.print("[bold red]ERROR: The query you passed is not a valid SELECT query for Postgres[/]\nPlease try with a different one.");
            }
        }
    }
}