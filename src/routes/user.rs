use rocket::fairing::AdHoc;
use rocket::response::status::BadRequest;
use rocket::{response::status::Unauthorized, serde::json::Json};

use rocket::{post, routes, State};

use crate::database::{models, rows, DatabaseAccountingApi};

use crate::types::response::Response;

#[post("/login", format = "application/json", data = "<user>")]
pub async fn login_user(
    user: Json<models::User>,
    storage: &State<DatabaseAccountingApi>,
) -> Result<Json<Response<models::User>>, Unauthorized<String>> {
    let user = &user.user;
    let user = sqlx::query_as!(
        rows::User,
        r#"
        SELECT id AS "id: _", name AS "name: _", password AS "password: _"
        FROM users
        WHERE name = $1 AND password = $2
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
pub async fn register_user(
    user: Json<models::User>,
    storage: &State<DatabaseAccountingApi>,
) -> Result<Json<Response<models::User>>, BadRequest<Json<Response<models::User>>>> {
    let user = &user.user;
    let user = sqlx::query_as!(
        rows::User,
        r#"
            INSERT INTO users (name, password)
            VALUES ($1, $2)
            RETURNING id AS "id: _", name AS "name: _", password AS "password: _"
        "#,
        user.name,
        user.password
    )
    .fetch_one(&storage.db)
    .await
    .map_err(|e| {
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

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("users stage", |rocket| async {
        rocket.mount("/api/users", routes![register_user, login_user])
    })
}
