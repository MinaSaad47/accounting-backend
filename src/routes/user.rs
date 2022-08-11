use rocket::{response::status::Unauthorized, serde::json::Json};

use rocket::{post, State};

use crate::{db::Storage, models::user::User};

use crate::types::uuid::Uuid;

#[post("/login", format = "application/json", data = "<user>")]
pub async fn login(
    user: Json<User>,
    storage: &State<Storage>,
) -> Result<Json<User>, Unauthorized<String>> {
    let user: User = sqlx::query_as!(
        User,
        r#"
        SELECT id AS "id: Uuid", name AS "name: String", password AS "password: String"
        FROM users
        WHERE name = ? AND password = ?
        "#,
        user.name,
        user.password
    )
    .fetch_one(&storage.db)
    .await
    .map_err(|e| Unauthorized(Some(format!("{e}"))))?;
    Ok(Json(user))
}
