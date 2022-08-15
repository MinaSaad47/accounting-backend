use rocket::response::status::BadRequest;
use rocket::{response::status::Unauthorized, serde::json::Json};

use rocket::{post, State};

use crate::db::Querable;
use crate::{
    db::{rows, Storage},
    models,
};

use crate::types::{response::Response, uuid::Uuid};

#[post("/login", format = "application/json", data = "<user>")]
pub async fn login(
    user: Json<models::User>,
    storage: &State<Storage>,
) -> Result<Json<Response<models::User>>, Unauthorized<String>> {
    let user = &user.user;
    let user = sqlx::query_as!(
        rows::User,
        r#"
        SELECT id AS "id?: Uuid", name AS "name: String", password AS "password: String"
        FROM users
        WHERE name = ? AND password = ?
        "#,
        user.name,
        user.password
    )
    .fetch_one(&storage.db)
    .await
    .map_err(|e| Unauthorized(Some(format!("{e}"))))?;
    Ok(Json(Response {
        status: true,
        message: "تم التسجيل بنجاح".to_string(),
        data: Some(models::User { user }),
    }))
}

#[post("/register", format = "application/json", data = "<user>")]
pub async fn register(
    mut user: Json<models::User>,
    storage: &State<Storage>,
) -> Result<Json<Response<models::User>>, BadRequest<Json<Response<models::User>>>> {
    let user = &mut user.user;
    user.id = Some(Uuid::new());
    let user = user.insert_query("users", &storage).await.map_err(|e| {
        BadRequest(Some(Json(Response {
            status: false,
            message: format!("{e}"),
            data: None,
        })))
    })?;
    Ok(Json(Response {
        status: true,
        message: "تم التسجيل بنجاح".to_string(),
        data: Some(models::User { user }),
    }))
}
