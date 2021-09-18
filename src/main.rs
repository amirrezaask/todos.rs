use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Deserialize, Serialize, Debug)]
pub struct Todo {
    pub text: String,

    // #[serde(default)]
    checked: Option<bool>,
}
struct Database {
    actual: Mutex<Vec<Todo>>,
}

impl Database {
    fn new() -> Self {
        Database {
            actual: Mutex::new(Vec::new()),
        }
    }
}

#[get("/todos/{id}")]
async fn index(db: web::Data<Database>, id: web::Path<i32>) -> impl Responder {
    println!("{}", id);
    HttpResponse::Ok().json(&db.actual)
}

#[post("/todos")]
async fn new(/*db: web::Data<Database>,*/ todo: web::Json<Todo>) -> impl Responder {
    // let mut rows = db.actual.lock().unwrap();
    println!("{:?}", todo);
    // (*rows).push(todo.0);
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
