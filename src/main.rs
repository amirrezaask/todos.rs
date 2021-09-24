use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Row;
use serde::{Deserialize, Serialize};

static GET_ALL_TODOS: &str = r#"SELECT * FROM todos"#;
static ADD_TODO: &str = "INSERT INTO todos (title) VALUES (?1)";
type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

#[derive(Serialize, Deserialize)]
struct Todo {
    title: String,
    #[serde(default)]
    done: bool,
}
impl Todo {
    fn from_row(row: &Row) -> Self {
        Todo {
            title: row.get_unwrap("title"),
            done: row.get_unwrap("done"),
        }
    }
}
#[actix_web::post("/")]
async fn new(db: web::Data<Pool>, todo: web::Json<Todo>) -> impl Responder {
    let conn = db.get().unwrap();
    let mut stmt = conn.prepare(ADD_TODO).unwrap();
    db.get().unwrap().execute(ADD_TODO, &[&todo.title]).unwrap();

    HttpResponse::Created()
}
#[actix_web::get("/")]
async fn index(db: web::Data<Pool>) -> impl Responder {
    let conn = db.get().unwrap();
    let mut stmt = conn.prepare(GET_ALL_TODOS).unwrap();
    let mut rows = stmt.query([]).unwrap();
    let mut todos: Vec<Todo> = Vec::new();
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

    HttpResponse::Ok().json(todos)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = SqliteConnectionManager::file("todos.db");
    let pool = Pool::new(manager).unwrap();

    HttpServer::new(move || App::new().service(index).service(new).data(pool.clone()))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
