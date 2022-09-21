pub mod accounting_api;
pub mod local_storage;
pub mod routes;
pub mod types;
pub mod auth;
pub mod file_system;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("db/migrations");
