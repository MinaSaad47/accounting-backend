pub mod accounting_api_impl;
pub mod models;
pub use models::*;
pub use models::*;
use sqlx::pool::PoolOptions;

use rocket::{fairing::AdHoc, tokio::sync::RwLock};
use sqlx::{Pool, Postgres};
use std::env;

use crate::file_system::FileSystem;

pub type DB = Postgres;

#[derive(Debug)]
pub struct LocalStorageAccountingApi {
    pub db: Pool<DB>,
    pub fs: RwLock<FileSystem>,
}

impl LocalStorageAccountingApi {
    async fn new(db_url: &str, fs_path: &str) -> sqlx::Result<Self> {
        Ok(LocalStorageAccountingApi {
            db: PoolOptions::new()
                .max_connections(100)
                .connect(db_url)
                .await?,
            fs: RwLock::new(FileSystem::new(fs_path).await),
        })
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("database stage", |rocket| async {
        let storage = LocalStorageAccountingApi::new(
            &env::var("DATABASE_URL").expect("`DATABASE_URL` must be set"),
            &env::var("DATA_PATH").expect("`DATA_PATH` must be set"),
        )
        .await
        .expect("database connection");
        sqlx::migrate!()
            .run(&storage.db)
            .await
            .expect("migrations run");
        rocket.manage(storage)
    })
}
