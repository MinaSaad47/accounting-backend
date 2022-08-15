use std::env;

#[macro_use]
extern crate rocket;

use accounting_backend::{db::Storage, routes};
use sqlx::SqlitePool;

#[launch]
async fn rocket() -> _ {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").expect("`DATABASE_URL` must be set"))
        .await
        .expect("connection pool");
    sqlx::migrate!("db/migrations")
        .undo(&pool, 0)
        .await
        .expect("migrations undo");
    sqlx::migrate!("db/migrations")
        .run(&pool)
        .await
        .expect("migrations run");
    rocket::build().manage(Storage { db: pool }).mount(
        "/api",
        routes![
            routes::user::login,
            routes::company::get,
            routes::company::post,
            routes::company::put,
        ],
    )
}
