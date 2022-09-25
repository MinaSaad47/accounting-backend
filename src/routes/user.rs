use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

use rocket::{delete, get, patch, post, routes, State};

use crate::accounting_api::AcountingApi;
use crate::auth::{AGuard, ApiToken, UGuard};
use crate::local_storage::{LocalStorageAccountingApi, *};

use crate::types::response::{ResponseEnum, ResponseResult};

#[post("/login", format = "application/json", data = "<user>")]
pub async fn login_user(
    user: Json<LoginUser>,
    storage: &State<LocalStorageAccountingApi>,
) -> ResponseResult<ApiToken> {
    let user = storage.login_user(&user).await?;
    let token = ApiToken::generate(user.id, user.is_admin);
    Ok(ResponseEnum::ok(token, "تم تسجيل الدخول بنجاح".into()))
}

#[post("/", format = "application/json", data = "<user>")]
pub async fn register_user(
    user: Json<RegisterUser>,
    storage: &State<LocalStorageAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<User> {
    let user = storage.register_user(&user).await?;
    Ok(ResponseEnum::created(
        user,
        "تم تسجيل مستخدم جديد بنجاح".into(),
    ))
}

#[get("/")]
pub async fn get_users_admin(
    storage: &State<LocalStorageAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<Vec<User>> {
    let users = storage.get_users().await?;
    Ok(ResponseEnum::ok(users, "تم ايجاد مستخدمين".into()))
}

#[get("/", rank = 2)]
pub async fn get_users_user(
    storage: &State<LocalStorageAccountingApi>,
    _ug: UGuard,
) -> ResponseResult<Vec<User>> {
    let users = storage.get_users().await?;
    Ok(ResponseEnum::ok(users, "تم ايجاد مستخدمين".into()))
}

#[get("/current")]
pub async fn get_current_user(
    storage: &State<LocalStorageAccountingApi>,
    ug: UGuard,
) -> ResponseResult<User> {
    let user = storage.get_user(ug.0).await?;
    Ok(ResponseEnum::ok(user, "تم ايجاد مستخدمين".into()))
}

#[get("/current", rank = 2)]
pub async fn get_current_admin(
    storage: &State<LocalStorageAccountingApi>,
    ag: AGuard,
) -> ResponseResult<User> {
    let user = storage.get_user(ag.0).await?;
    Ok(ResponseEnum::ok(user, "تم ايجاد مستخدمين".into()))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Value {
    value: f64,
}

#[patch("/<id>", format = "application/json", data = "<value>")]
pub async fn pay_user(
    id: i64,
    value: Json<Value>,
    storage: &State<LocalStorageAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<User> {
    let user = storage.pay_user(id, value.value).await?;
    Ok(ResponseEnum::ok(user, "تم تعديل القيمة".into()))
}

#[delete("/<id>")]
pub async fn delete_user(
    id: i64,
    storage: &State<LocalStorageAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<()> {
    storage.delete_user(id).await?;
    Ok(ResponseEnum::ok((), "تم حذف المستخدم".into()))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("users stage", |rocket| async {
        rocket.mount(
            "/api/users",
            routes![
                register_user,
                login_user,
                get_users_user,
                get_users_admin,
                get_current_user,
                get_current_admin,
                pay_user,
                delete_user,
            ],
        )
    })
}
