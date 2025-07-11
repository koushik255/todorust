use clap::{Arg, command, parser::IdsRef};
use cli_table::{Cell, Style, Table, WithTitle, format::Justify, print_stdout};
use rand::Rng;
use std::sync::Arc;
use tokio_rusqlite::Connection;
mod db;

use db::{Todo, db, del_todo, insert_todo, list_todos};

fn input() -> Vec<String> {
    let matches = command!()
        .arg(
            Arg::new("input")
                .help("Your help input")
                .required(true)
                .num_args(1..),
        )
        .get_matches();

    matches
        .get_many::<String>("input")
        .unwrap()
        .cloned()
        .collect()
}

fn gen_id() -> i64 {
    let mut rng = rand::thread_rng();
    let random_number: i64 = rng.gen_range(0..1_000_000);
    println!("random number: {}", random_number);
    random_number
}

async fn receive(db: Arc<Connection>) -> Result<(), Box<dyn std::error::Error>> {
    let input_vec = input();

    match input_vec.get(0).map(|s| s.as_str()) {
        Some("add") => {
            println!("add func");
            println!("{:?}", input_vec);
            let toget = input_vec[1..].join(" ");
            println!(" Joined: {}", toget);
            let todo = Todo {
                id: gen_id(),
                text: toget,
            };
            insert_todo(todo.clone(), axum::Extension(db)).await;
            // i would add the database function here that would just consume the todo struct
            // i dont think it should be any harder than that?
            //
        }
        Some("show") => {
            println!("Show todos");
            let to_print = list_todos(axum::Extension(db)).await;
            let mut todo_print: Vec<Todo> = Vec::new();
            match to_print {
                Ok(todos) => {
                    for todo in todos {
                        println!("ID {}, TExt: {}", todo.id, todo.text);
                        todo_print.push(todo);

                        // MATCH STATEMENTS ONLY AND RESULT TYPES AND OK TYPES
                    }
                    assert!(print_stdout(todo_print.with_title()).is_ok());
                }

                Err(e) => {
                    println!("errpr {:?}", e);
                }
            }
        }
        Some("del") => {
            println!("why del blud");
            println!("{:?}", input_vec);
            let id = input_vec[1].parse::<i64>().unwrap();
            println!(" todo to delete is: {:?}", id);
            del_todo(id, axum::Extension(db)).await;
            println!("delete todo: {id}");
        }

        Some(other) => {
            println!("Something else: {}", other);
        }
        None => {
            println!("No input provided");
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = db().await?;
    receive(db).await.unwrap();

    Ok(())
}
