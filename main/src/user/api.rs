use std::sync::Arc;

use sqlx::Pool;
#[cfg(feature = "db-pg")]
use sqlx::Postgres;
#[cfg(feature = "db-sqlite")]
use sqlx::Sqlite;

use crate::{
    types::AppResult,
    user::{Repository, domain::User},
};

#[derive(Clone)]
pub struct Service {
    repo: Arc<dyn Repository>,
}

impl Service {
    pub fn new(repo: impl Repository) -> Self {
        Self { repo: Arc::new(repo) }
    }

    pub async fn find_by_id(&self, id: &u64) -> AppResult<Option<User>> {
        self.repo.find_by_id(id).await
    }

    pub async fn create(&self, user: User) -> AppResult<User> {
        self.repo.insert(user).await
    }

    pub async fn list(&self) -> AppResult<Vec<User>> {
        self.repo.list().await
    }

    pub async fn update(&self, id: u64, user: User) -> AppResult<Option<User>> {
        self.repo.update(id, user).await
    }

    pub async fn delete(&self, id: &u64) -> AppResult<bool> {
        self.repo.delete(id).await
    }
}

#[cfg(feature = "db-pg")]
impl Service {
    pub fn new_pg(db: Pool<Postgres>) -> Self {
        use crate::user::repository::pg::PgRepository;

        let repo = PgRepository(db);
        Self::new(repo)
    }
}

#[cfg(feature = "db-sqlite")]
impl Service {
    pub fn new_sqlite(db: Pool<Sqlite>) -> Self {
        use crate::user::repository::sqlite::SqliteRepository;

        let repo = SqliteRepository(db);
        Self::new(repo)
    }
}
