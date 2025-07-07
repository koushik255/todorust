use axum::extract::Extension;
use std::sync::Arc;
use tokio_rusqlite::Connection;

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: i64,
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

pub async fn list_todos(Extension(db): Extension<Db>) -> Vec<String> {
    let todo = db
        .call(|conn| {
            let mut stmt = conn.prepare("SELECT id,text FROM todos")?;
            let rows = stmt
                .query_map([], |row| {
                    Ok(Todo {
                        id: row.get(0)?,
                        text: row.get(1)?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(rows)
        })
        .await;

    match todo {
        Ok(todo) => {
            if todo.is_empty() {
                vec![" people found".to_string()]
            } else {
                todo.into_iter()
                    .map(|t| format!("{}: {}", t.id, t.text))
                    .collect::<Vec<_>>()
            }
        }
        Err(e) => vec![format!("DB errror: {}", e,)],
    }
}
