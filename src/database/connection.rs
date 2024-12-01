// Database wrapper
use duckdb::{params, Connection, Error, Result};

pub struct Database {
    pub(in crate::database) conn: Connection,
}

impl Database {
    pub fn new_inmemory() -> Result<Database, Error> {
        Connection::open_in_memory()
            .and_then(|conn: Connection| {
                conn.execute_batch(
                    r"
                    CREATE SEQUENCE account_seq;
                    CREATE TABLE Users (
                        user_id             TEXT PRIMARY KEY NOT NULL,
                        name                TEXT NOT NULL,
                        accounts            BLOB NOT NULL
                    );
        
                    CREATE TABLE Accounts (
                        account_id          INTEGER PRIMARY KEY DEFAULT NEXTVAL('account_seq'),
                        account_type        TEXT NOT NULL,
                        balance             DOUBLE NOT NULL,
                    );",
                )
                .and_then(|_| Ok(conn))
            })
            .and_then(|conn: Connection| Ok(Database { conn }))
    }

    pub fn add_user(&self, user: User) -> Result<usize, Error> {
        self.conn.execute(
            "INSERT INTO Users (user_id, name, accounts) VALUES (?, ?, ?) ON CONFLICT DO NOTHING",
            params![user.user_id, user.name, user.accounts],
        )
    }

    pub fn get_user(&self, user_id: String) -> Result<User, Error> {
        self.conn.query_row(
            "SELECT * FROM Users WHERE user_id = ?",
            params![user_id],
            |row| {
                Ok(User {
                    user_id: row.get(0)?,
                    name: row.get(1)?,
                    accounts: row.get(2)?,
                })
            },
        )
    }
}

#[derive(Debug)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub accounts: Vec<u8>,
}

pub enum AccountType {
    CHEQUING,
    SAVINGS,
}

pub struct Account {
    pub account_id: u8,
    pub account_type: AccountType,
    pub balance: f64,
}
