pub mod accounting_api_impl;
pub mod models;
pub mod rows;
pub use models::*;
pub use rows::*;
use sqlx::pool::PoolOptions;

use rocket::fairing::AdHoc;
use sqlx::{Pool, Postgres};
use std::env;

pub type DB = Postgres;

#[derive(Debug)]
pub struct DatabaseAccountingApi {
    pub db: Pool<DB>,
}

impl DatabaseAccountingApi {
    async fn connect(url: &str) -> sqlx::Result<Self> {
        Ok(DatabaseAccountingApi {
            db: PoolOptions::new().max_connections(100).connect(url).await?,
        })
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("database stage", |rocket| async {
        let storage = DatabaseAccountingApi::connect(
            &env::var("DATABASE_URL").expect("`DATABASE_URL` must be set"),
        )
        .await
        .expect("database connection");
        sqlx::migrate!("db/migrations")
            .run(&storage.db)
            .await
            .expect("migrations run");
        rocket.manage(storage)
    })
}
