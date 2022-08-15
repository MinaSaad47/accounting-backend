pub mod querable;
pub mod rows;

use std::env;

pub use querable::*;
use rocket::fairing::AdHoc;
pub use rows::*;

use sqlx::{Pool, Sqlite, SqlitePool};

type DBPool = Pool<Sqlite>;

#[derive(Debug)]
pub struct Storage {
    pub db: DBPool,
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("database stage", |rocket| async {
        let pool =
            SqlitePool::connect(&env::var("DATABASE_URL").expect("`DATABASE_URL` must be set"))
                .await
                .expect("database connection");
        sqlx::migrate!("db/migrations")
            .undo(&pool, 0)
            .await
            .expect("migrations undo");
        sqlx::migrate!("db/migrations")
            .run(&pool)
            .await
            .expect("migrations run");
        rocket.manage(Storage { db: pool })
    })
}
