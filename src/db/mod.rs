pub mod rows;
pub mod querable;


pub use rows::*;
pub use querable::*;

use sqlx::{Pool, Sqlite};

type DBPool = Pool<Sqlite>;

#[derive(Debug)]
pub struct Storage {
    pub db: DBPool,
}
