use rocket::{delete, fairing::AdHoc, get, routes, FromForm, State};

use crate::{
    accounting_api::AcountingApi,
    auth::AGuard,
    database::{models, DatabaseAccountingApi},
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
    id: i64,
}

#[derive(Debug, FromForm, PartialEq)]
#[allow(dead_code)]
struct User {
    id: i64,
}

#[get("/?<param..>")]
pub async fn get_incomes(
    param: GetParam,
    storage: &State<DatabaseAccountingApi>,
) -> ResponseResult<Vec<models::Income>> {
    rocket::debug!("{param:?}");
    let incomes = storage
        .get_incomes(param.admin.map(|u| u.id), param.company.map(|c| c.id))
        .await?;
    Ok(ResponseEnum::ok(incomes, "تم ايجاد رؤؤوس اموال".into()))
}

#[delete("/<id>")]
pub async fn delete_income(
    id: i64,
    storage: &State<DatabaseAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<()> {
    storage.delete_income(id).await?;
    Ok(ResponseEnum::ok((), "تم مسح رأس المال: {id}".into()))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("incomes stage", |rocket| async {
        rocket.mount("/api/incomes", routes![get_incomes, delete_income])
    })
}
