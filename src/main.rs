#[macro_use]
extern crate rocket;

use rocket::{http::Status, serde::json::Json};

use rocket_sync_db_pools::{database, rusqlite::Row};

use serde::{Deserialize, Serialize};

static GET_ALL_TODOS: &str = r#"SELECT * FROM todos"#;
static ADD_TODO: &str = "INSERT INTO todos (title) VALUES (?1)";

type Connection = rocket_sync_db_pools::rusqlite::Connection;

#[database("todos")]
struct TodosDb(Connection);

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub title: String,
    #[serde(default)]
    pub done: bool,
}

impl Todo {
    pub fn new(title: String) -> Todo {
        Todo { title, done: false }
    }
    pub fn from_row(row: &Row) -> Self {
        Todo {
            title: row.get_unwrap(1),
            done: row.get_unwrap(2),
        }
    }
}

#[get("/todos")]
async fn list(db: TodosDb) -> Json<Vec<Todo>> {
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
