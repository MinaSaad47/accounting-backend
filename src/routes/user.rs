use rocket::fairing::AdHoc;
use rocket::serde::json::Json;

use rocket::{get, post, routes, State};

use crate::accounting_api::AcountingApi;
use crate::auth::{AGuard, ApiToken, UGuard};
use crate::database::{models, DatabaseAccountingApi};

use crate::types::response::{ResponseEnum, ResponseResult};

#[post("/login", format = "application/json", data = "<user>")]
pub async fn login_user(
    user: Json<models::User>,
    storage: &State<DatabaseAccountingApi>,
) -> ResponseResult<ApiToken> {
    let user = storage.login_user(&user).await?;
    let c = &user.user;
    let token = ApiToken::generate(c.id.expect("valid user id"), c.is_admin);
    Ok(ResponseEnum::ok(token, "تم تسجيل الدخول بنجاح".into()))
}

#[post("/", format = "application/json", data = "<user>")]
pub async fn create_user(
    user: Json<models::User>,
    storage: &State<DatabaseAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<models::User> {
    let user = storage.create_user(&user).await?;
    Ok(ResponseEnum::created(
        user,
        "تم تسجيل مستخدم جديد بنجاح".into(),
    ))
}

#[get("/")]
pub async fn get_users_admin(
    storage: &State<DatabaseAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<Vec<models::User>> {
    let users = storage.get_users().await?;
    Ok(ResponseEnum::ok(users, "تم ايجاد مستخدمين".into()))
}

#[get("/", rank = 2)]
pub async fn get_users_user(
    storage: &State<DatabaseAccountingApi>,
    _ug: UGuard,
) -> ResponseResult<Vec<models::User>> {
    let users = storage.get_users().await?;
    Ok(ResponseEnum::ok(users, "تم ايجاد مستخدمين".into()))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("users stage", |rocket| async {
        rocket.mount(
            "/api/users",
            routes![create_user, login_user, get_users_user, get_users_admin],
        )
    })
}
