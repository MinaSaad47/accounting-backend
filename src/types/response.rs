use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    pub status: bool,
    pub message: String,
    pub data: Option<T>
}
