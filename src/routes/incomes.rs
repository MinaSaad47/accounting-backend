use rocket::{delete, fairing::AdHoc, get, routes, FromForm, State};
use sqlx::types::Uuid;

use crate::{
    accounting_api::AcountingApi,
    auth::AGuard,
    local_storage::{models, LocalStorageAccountingApi},
    types::response::{ResponseEnum, ResponseResult},
};

#[derive(Debug, FromForm, PartialEq)]
#[allow(dead_code)]
pub struct GetParam {
    company: Option<Company>,
    admin: Option<User>,
}

#[derive(Debug, FromForm, PartialEq)]
#[allow(dead_code)]
struct Company {
    id: Uuid,
}

#[derive(Debug, FromForm, PartialEq)]
#[allow(dead_code)]
struct User {
    id: Uuid,
}

#[get("/?<param..>")]
pub async fn get_incomes(
    param: GetParam,
    storage: &State<LocalStorageAccountingApi>,
) -> ResponseResult<Vec<models::Income>> {
    rocket::debug!("{param:?}");
    let incomes = storage
        .get_incomes(param.admin.map(|u| u.id), param.company.map(|c| c.id))
        .await?;
    Ok(ResponseEnum::ok(incomes, "تم ايجاد رؤؤوس اموال".into()))
}

#[delete("/<id>")]
pub async fn delete_income(
    id: Uuid,
    storage: &State<LocalStorageAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<()> {
    storage.delete_income(id).await?;
    Ok(ResponseEnum::ok((), "تم مسح واردات".into()))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("incomes stage", |rocket| async {
        rocket.mount("/api/incomes", routes![get_incomes, delete_income])
    })
}
