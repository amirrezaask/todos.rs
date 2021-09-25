# Todos.rs
Simple Todo API, create and read all.

- [actix-web](https://actix.rs/) + [rusqlite](https://github.com/rusqlite/rusqlite): [Implementation](https://github.com/amirrezaask/todos.rs/tree/actix-web-rusqlite)
- [rocket](https://rocket.rs/) + [rusqlite](https://github.com/rusqlite/rusqlite): [Implementation](https://github.com/amirrezaask/todos.rs/tree/rocket-rusqlite)

- [actix-web](https://actix.rs/) + [sqlx](https://github.com/launchbadge/sqlx): [Implementation](https://github.com/amirrezaask/todos.rs/tree/actix-web-sqlx)
- [rocket](https://rocket.rs/) + [sqlx](https://github.com/launchbadge/sqlx): [Implementation](https://github.com/amirrezaask/todos.rs/tree/rocket-sqlx)


# Benchamrks
| Libs          | RPS             
| ------------- | -------------
| Actix web + sqlx      | 5172.07
| Actix web + rusqlite      | 20270.46 
| Rocket + rusqlite | 6973.83
| Rocket + sqlx | 6127.07
