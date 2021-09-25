use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

static GET_ALL_TODOS: &str = r#"SELECT * FROM todos"#;
type Pool = sqlx::Pool<sqlx::sqlite::Sqlite>;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct Todo {
    title: String,
    #[serde(default)]
    done: bool,
}

#[actix_web::post("/")]
async fn new(db: web::Data<Pool>, todo: web::Json<Todo>) -> impl Responder {
    let mut conn = db.acquire().await.unwrap();
    let todo_id = sqlx::query("INSERT INTO todos (title) VALUES (?)")
        .bind(&todo.title)
        .execute(&mut conn)
        .await
        .unwrap()
        .last_insert_rowid();
    HttpResponse::Created().body(format!("{}", todo_id))
}

#[actix_web::get("/")]
async fn index(db: web::Data<Pool>) -> impl Responder {
    let mut conn = db.acquire().await.unwrap();
    let todos: Vec<Todo> = sqlx::query_as(GET_ALL_TODOS)
        .fetch_all(&mut conn)
        .await
        .unwrap();
    HttpResponse::Ok().json(todos)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = SqlitePool::connect("todos.db").await.unwrap();

    HttpServer::new(move || App::new().service(index).service(new).data(pool.clone()))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
