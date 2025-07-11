use axum::extract::Extension;
use cli_table::{Cell, Style, Table, WithTitle, format::Justify, print_stdout};
use std::sync::Arc;
use tokio_rusqlite::{Connection, Error};

#[derive(Table, Debug, Clone)]
pub struct Todo {
    #[table(title = "ID", justify = "Justify::Right")]
    pub id: i64,
    #[table(title = "text")]
    pub text: String,
}

pub type Db = Arc<Connection>;

pub async fn insert_todo(todo: Todo, Extension(db): Extension<Db>) -> String {
    let id = todo.id;
    let text = todo.text;

    let insert = db
        .call(move |conn| {
            Ok(conn.execute("INSERT INTO todos (id, text) VALUES (?1,?2)", (&id, &text))?)
        })
        .await;

    match insert {
        Ok(_) => format!(": {:?}", &id),
        Err(e) => format!("DB error: {}", e),
    }
}

// if the id is simmilar to one of ones delete is

pub async fn del_todo(todoid: i64, Extension(db): Extension<Db>) -> String {
    let id = todoid;

    let delete = db
        .call(move |conn| Ok(conn.execute("DELETE FROM todos WHERE id = ?1", [&id])?))
        .await;

    match delete {
        Ok(rows_affect) => {
            if rows_affect > 0 {
                format!("Deleted todo with id: {}", id)
            } else {
                format!("No todo found with id {}", id)
            }
        }
        Err(e) => format!("DB error {}", e),
    }
}

pub async fn db() -> Result<Db, Box<dyn std::error::Error>> {
    let db = Connection::open("my_db").await?;
    db.call(|conn| {
        Ok(conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY,
                text TEXT NOT NULL,
                data BLOB
            ) STRICT",
            (),
        )?)
    })
    .await?;
    Ok(Arc::new(db))
}

pub async fn list_todos(Extension(db): Extension<Db>) -> Result<Vec<Todo>, Vec<String>> {
    let todo = db
        .call(|conn| {
            let mut stmt = conn.prepare("SELECT id,text FROM todos")?;
            let rows = stmt.query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    text: row.get(1)?,
                })
            })?;
            let todos: Result<Vec<Todo>, _> = rows.collect();

            Ok(todos)
        })
        .await;

    match todo {
        Ok(todo_inner) => match todo_inner {
            Ok(todos) => {
                if todos.is_empty() {
                    Err(vec![format!("no todos found")])
                } else {
                    Ok(todos)
                }
            }
            Err(e) => Err(vec![format!("Databquea erro {}", e)]),
        },
        Err(e) => Err(vec![format!("DB errror: {}", e,)]),
    }
}
