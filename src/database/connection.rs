// Database wrapper
use duckdb::{params, types::FromSql, Connection, Error, Result, ToSql};

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
                    );
        
                    CREATE TABLE Accounts (
                        account_id          INTEGER PRIMARY KEY DEFAULT NEXTVAL('account_seq'),
                        user_id             TEXT NOT NULL,
                        account_type        TEXT NOT NULL,
                        balance             DOUBLE NOT NULL,
                    );
                    CREATE INDEX account_user_idx ON Accounts (user_id);",
                )
                .and_then(|_| Ok(conn))
            })
            .and_then(|conn: Connection| Ok(Database { conn }))
    }

    pub fn add_user(&self, user: User) -> Result<usize, Error> {
        self.conn.execute(
            "INSERT INTO Users (user_id, name) VALUES (?, ?) ON CONFLICT DO NOTHING",
            params![user.user_id, user.name],
        )
    }

    pub fn get_accounts(&self, user_id: &String) -> Result<UserSummary, Error> {
        // First get the User object.
        let user: Result<User, Error> = self.conn.query_row(
            "SELECT * FROM Users WHERE user_id = ?",
            params![user_id],
            |row| {
                Ok(User {
                    user_id: row.get::<_, String>(0)?,
                    name: row.get::<_, String>(1)?,
                })
            },
        );
        if user.is_err() {
            return Err(user.unwrap_err());
        }

        // Now get all accounts for this user.
        let mut stmt: duckdb::Statement<'_> = self
            .conn
            .prepare("SELECT * FROM Accounts WHERE user_id = ?")?;
        let accounts_it = stmt.query_map(params![user_id], |row| {
            Ok(Account {
                account_id: row.get::<_, u8>(0)?,
                user_id: row.get::<_, String>(1)?,
                account_type: AccountType::from_string(row.get::<_, String>(2)?)?,
                balance: row.get::<_, f64>(0)?,
            })
        })?;

        let accounts: Vec<Account> = accounts_it.map(|account| account.unwrap()).collect();
        Ok(UserSummary {
            user: user?,
            accounts,
        })
    }

    pub fn create_account(&self, user: &User, account_type: AccountType) -> Result<(), Error> {
        let result: Result<usize, Error> = self.conn.execute(
            "INSERT INTO Accounts (user_id, account_type, balance) VALUES (?, ?, ?)",
            params![user.user_id, account_type, 0.0],
        );

        match result {
            Err(error) => Err(error),
            Ok(_) => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct User {
    pub user_id: String,
    pub name: String,
}

pub enum AccountType {
    CHEQUING,
    SAVINGS,
}

impl ToSql for AccountType {
    fn to_sql(&self) -> Result<duckdb::types::ToSqlOutput<'_>> {
        match self {
            AccountType::CHEQUING => Ok(duckdb::types::ToSqlOutput::from("CHEQUING")),
            AccountType::SAVINGS => Ok(duckdb::types::ToSqlOutput::from("CHEQUING")),
        }
    }
}

impl AccountType {
    fn from_string(str: String) -> Result<AccountType, Error> {
        match str.as_str() {
            "CHEQUING" => Ok(AccountType::CHEQUING),
            "SAVINGS" => Ok(AccountType::SAVINGS),
            _ => Err(Error::InvalidParameterName(str.to_string())),
        }
    }
}

pub struct Account {
    pub account_id: u8,
    pub user_id: String,
    pub account_type: AccountType,
    pub balance: f64,
}

pub struct UserSummary {
    pub user: User,
    pub accounts: Vec<Account>,
}
