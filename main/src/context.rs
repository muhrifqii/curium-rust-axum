use std::sync::Arc;

use crate::{
    infrastructure::{config, db, logger},
    types::AppResult,
    user,
};

#[derive(Clone)]
pub struct AppApiCtx {
    pub user: Arc<user::Service>,
}

#[derive(Clone)]
pub struct AppCtx {
    pub is_production: bool,
    pub config: config::Config,
    pub api: AppApiCtx,
}

impl AppCtx {
    pub async fn init() -> AppResult<Self> {
        let (config, is_production) = config::Config::from_profile();
        logger::init(&config.logger)?;

        #[cfg(feature = "db-pg")]
        let user_api = {
            let db = db::init_pg(&config.db.url()).await;
            user::Service::new_pg(db)
        };

        #[cfg(feature = "db-sqlite")]
        let user_api = {
            let db = db::init_sqlite(&config.db.url()).await;
            user::Service::new_sqlite(db)
        };

        let api = AppApiCtx { user: Arc::new(user_api) };

        Ok(Self { is_production, config, api })
    }
}
