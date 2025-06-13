use super::domain::User;
use crate::types::AppResult;
use async_trait::async_trait;
use sqlx::{Database, Pool};

#[async_trait]
pub trait Repository: Send + Sync + 'static {
    async fn find_by_id(&self, id: &u64) -> AppResult<Option<User>>;
    async fn list(&self) -> AppResult<Vec<User>>;
    async fn create(&self, user: User) -> AppResult<User>;
    async fn update(&self, id: u64, user: User) -> AppResult<Option<User>>;
    async fn delete(&self, id: &u64) -> AppResult<bool>;
}

#[cfg(feature = "db-sqlite")]
pub mod sqlite {
    use super::*;
    use sqlx::Sqlite;

    #[derive(Clone)]
    pub struct SqliteRepository(pub(crate) Pool<Sqlite>);

    #[async_trait]
    impl Repository for SqliteRepository {
        async fn find_by_id(&self, id: &u64) -> AppResult<Option<User>> {
            todo!()
        }

        async fn list(&self) -> AppResult<Vec<User>> {
            todo!()
        }

        async fn create(&self, user: User) -> AppResult<User> {
            todo!()
        }

        async fn update(&self, id: u64, user: User) -> AppResult<Option<User>> {
            todo!()
        }

        async fn delete(&self, id: &u64) -> AppResult<bool> {
            todo!()
        }
    }
}

#[cfg(feature = "db-pg")]
pub mod pg {
    use super::*;
    use sqlx::Postgres;

    #[derive(Clone)]
    pub struct PgRepository(pub(crate) Pool<Postgres>);

    #[async_trait]
    impl Repository for PgRepository {
        async fn find_by_id(&self, id: &u64) -> AppResult<Option<User>> {
            todo!()
        }

        async fn list(&self) -> AppResult<Vec<User>> {
            todo!()
        }

        async fn create(&self, user: User) -> AppResult<User> {
            todo!()
        }

        async fn update(&self, id: u64, user: User) -> AppResult<Option<User>> {
            todo!()
        }

        async fn delete(&self, id: &u64) -> AppResult<bool> {
            todo!()
        }
    }
}
