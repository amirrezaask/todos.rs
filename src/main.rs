#[macro_use]
extern crate rocket;

use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::Sqlite;

use std::sync::Arc;

static GET_ALL_TODOS: &str = r#"SELECT * FROM todos"#;
static ADD_TODO: &str = "INSERT INTO todos (title) VALUES (?1)";

type Db = sqlx::Pool<Sqlite>;

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Todo {
    pub title: String,
    #[serde(default)]
    pub done: bool,
}

#[get("/")]
async fn list(db: &State<Db>) -> Json<Vec<Todo>> {
    let mut conn = db.acquire().await.unwrap();
    let todos: Vec<Todo> = sqlx::query_as(GET_ALL_TODOS)
        .fetch_all(&mut conn)
        .await
        .unwrap();

    Json(todos)
}

#[post("/", data = "<todo>")]
async fn add(db: &State<Db>, todo: Json<Todo>) -> Status {
    let mut conn = db.acquire().await.unwrap();
    let todo_id = sqlx::query("INSERT INTO todos (title) VALUES (?)")
        .bind(&todo.title)
        .execute(&mut conn)
        .await
        .unwrap()
        .last_insert_rowid();
    Status::Created
}

#[launch]
async fn rocket() -> _ {
    let pool = sqlx::sqlite::SqlitePool::connect("todos.db").await.unwrap();
    rocket::build()
        .manage(pool.clone())
        .mount("/", routes![list, add])
}
