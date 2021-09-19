#[macro_use]
extern crate rocket;

mod models;
use models::Todo;
use rocket::serde::json::Json;
use rocket::State;
use std::sync::Arc;
use std::sync::RwLock;

struct TodosState(Arc<RwLock<Vec<Todo>>>);

#[get("/todos")]
async fn list(state: &State<TodosState>) -> Json<Vec<Todo>> {
    let data = &*(state.0.read().unwrap());
    Json(data.to_vec())
}

#[post("/todos", data = "<todo>")]
async fn add(state: &State<TodosState>, todo: Json<Todo>) -> Json<Todo> {
    let mut d = state.0.write().unwrap();
    d.push(todo.0.clone());
    Json(todo.0)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(TodosState(Arc::new(RwLock::new(vec![Todo::new(
            String::from("First todo"),
        )]))))
        .mount("/", routes![list, add])
}
