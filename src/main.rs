use std::env;

#[macro_use]
extern crate rocket;

use accounting_backend::{db::Storage, routes};
use sqlx::SqlitePool;

#[launch]
async fn rocket() -> _ {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    rocket::build().manage(Storage { db: pool }).mount(
        "/api",
        routes![
            routes::user::login,
            routes::company::get,
            routes::company::post,
        ],
    )
}
