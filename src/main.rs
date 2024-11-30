use duckdb::{params, Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() {
    println!("Hello, world!");

    let conn: Connection = Connection::open_in_memory().expect("Failed to open database.");

    conn.execute_batch(
        r"CREATE SEQUENCE seq;
          CREATE TABLE person (
                  id              INTEGER PRIMARY KEY DEFAULT NEXTVAL('seq'),
                  name            TEXT NOT NULL,
                  data            BLOB
                  );
        ",
    )
    .expect("Failed to create DB");

    let name: String = String::from("Ryan");
    let vec: Vec<u8> = vec![1,2];

    conn.execute(
        "INSERT INTO person (name, data) VALUES (?, ?)",
        params![name, vec],
    )
    .expect("Failed to execute insert");

    let mut stmt: duckdb::Statement<'_> = conn
        .prepare("SELECT id, name, data FROM person")
        .expect("Invalid statement");
    let person_iter = stmt
        .query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                data: row.get(2)?,
            })
        })
        .expect("Failed to get rows.");

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
}
