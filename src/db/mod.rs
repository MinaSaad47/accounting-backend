pub mod rows;

pub use rows::*;

use sqlx::{Pool, Sqlite};

type DBPool = Pool<Sqlite>;

#[derive(Debug)]
pub struct Storage {
    pub db: DBPool,
}
