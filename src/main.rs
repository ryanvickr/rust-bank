use duckdb::{params, Connection, Result};

fn main() {
    println!("Hello, world!");

    let _conn: Connection = Connection::open_in_memory().expect("Failed to open database.");
}
