#[macro_use]
extern crate rocket;

use rocket::{http::Status, serde::json::Json};

use serde::{Deserialize, Serialize};

static GET_ALL_TODOS: &str = r#"SELECT * FROM todos"#;
static ADD_TODO: &str = "INSERT INTO todos (title) VALUES (?1)";

#[derive(sqlx::Database)]
#[database("sqlx")]
struct Db(sqlx::SqlitePool);

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Todo {
    pub title: String,
    #[serde(default)]
    pub done: bool,
}

#[get("/todos")]
async fn list(db: Db) -> Json<Vec<Todo>> {
    let todos = db
        .run(|c| {
            let mut stmt = c.prepare(GET_ALL_TODOS).unwrap();
            let mut rows = stmt.query([]).unwrap();
            let mut todos = Vec::new();
            loop {
                match rows.next() {
                    Ok(opt) => match opt {
                        Some(row) => todos.push(Todo::from_row(row)),
                        None => break,
                    },
                    Err(err) => {
                        println!("err {}", err);
                        break;
                    }
                }
            }
            todos
        })
        .await;

    Json(todos)
}

#[post("/todos", data = "<todo>")]
async fn add(db: TodosDb, todo: Json<Todo>) -> Status {
    db.run(move |c| {
        c.prepare(ADD_TODO)
            .unwrap()
            .execute(&[&todo.0.title])
            .unwrap()
    })
    .await;

    Status::Created
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(TodosDb::fairing())
        .mount("/", routes![list, add])
}
