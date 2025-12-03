use rusqlite::Connection;
use std::rc::Rc;
use std::sync::LazyLock;

pub mod project;

pub type Result<T> = std::result::Result<T, rusqlite::Error>;

pub fn get_conn() -> Connection {
    let home = std::env::home_dir().unwrap();
    let db_path = home.join(".Seeker/seeker.db");

    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create .Seeker directory");
    }
    let connection_string = format!("{}", db_path.to_string_lossy());
    Connection::open(&connection_string).expect("Failed to initialize SQLite pool")
}
