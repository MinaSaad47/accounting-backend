pub mod accounting_api;
pub mod database;
pub mod routes;
pub mod types;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("db/migrations");
