use std::str::FromStr;

use sqlx::Pool;

#[cfg(feature = "db-pg")]
use sqlx::{Postgres, postgres::PgPoolOptions};

#[cfg(feature = "db-pg")]
pub async fn init_pg(db_url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await
        .expect("Could not connect to pg database")
}

#[cfg(feature = "db-sqlite")]
use sqlx::{
    Sqlite,
    sqlite::{
        SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions,
        SqliteSynchronous,
    },
};

#[cfg(feature = "db-sqlite")]
pub async fn init_sqlite(db_url: &str) -> Pool<Sqlite> {
    let opts = SqliteConnectOptions::from_str(db_url)
        .expect("Could not create sqlite database connection from given url")
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true)
        .synchronous(SqliteSynchronous::Normal)
        .pragma("mmap_size", "52428800"); // 50MB

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await
        .expect("Could not connect to sqlite database")
}
