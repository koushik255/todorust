use clap::{Arg, command};
use rand::Rng;
use std::sync::Arc;
use tokio_rusqlite::Connection;

mod db;

use db::{Todo, db, insert_todo, list_todos};

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

            match to_print {
                Ok(todos) => {
                    for todo in todos {
                        println!("ID {}, TExt: {}", todo.id, todo.text);
                        // MATCH STATEMENTS ONLY AND RESULT TYPES AND OK TYPES
                    }
                }

                Err(e) => {
                    println!("errpr {:?}", e);
                }
            }
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
