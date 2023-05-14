use std::fmt::Debug;
use derive_more::{Display, From};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::str::FromStr;
use sqlx::Sqlite;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error)
}

pub type AppDatabase = Database<Sqlite>;
pub type DatabasePool = sqlx::sqlite::SqlitePool;
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
pub type AppQueryResult = sqlx::sqlite::SqliteQueryResult;

pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Sqlite> {
    pub async fn new(connection_str: &str) -> Self {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(connection_str)
            .await;
        match pool {
            Ok(pool) => Self(pool),
            Err(e) => {
                eprintln!("{}\n", e);
                eprintln!("If database has not yet been created, run: \n $ sqlx database setup");
                panic!("database connection error!")
            }
        }
    }

    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}


#[derive(Clone, Debug, From, Display, Serialize, Deserialize)]
pub struct Dbid(Uuid);

impl Dbid {
    pub fn new() -> Dbid {
        Uuid::new_v4().into()
    }

    pub fn nil() -> Dbid {
        Self(Uuid::nil())
    }
}

impl Default for Dbid {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Dbid {
    type Err = uuid::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(Dbid(Uuid::parse_str(id)?))
    }
}