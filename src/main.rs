use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Deserialize, Serialize, Debug)]
pub struct Todo {
    pub text: String,
    checked: Option<bool>,
}

struct Database(Mutex<Vec<Todo>>);

impl Database {
    fn new() -> Self {
        Database(Mutex::new(Vec::new()))
    }
}

#[get("/todos")]
async fn index(db: web::Data<Database>) -> impl Responder {
    println!("{:?}", db.0);
    HttpResponse::Ok().json(&db.0)
}

#[post("/todos")]
async fn new(db: web::Data<Database>, mut todo: web::Json<Todo>) -> impl Responder {
    let mut rows = db.0.lock().unwrap();
    todo.0.checked = None;
    (*rows).push(todo.0);
    HttpResponse::Created()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(new)
            .app_data(web::Data::new(Database::new()))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
