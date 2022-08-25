use rocket::{delete, fairing::AdHoc, get, routes, FromForm, State};

use crate::{
    accounting_api::AcountingApi,
    auth::UGuard,
    database::{models, DatabaseAccountingApi},
    types::response::{ResponseEnum, ResponseResult},
};

#[derive(Debug, FromForm, PartialEq)]
#[allow(dead_code)]
pub struct GetParam {
    company: Option<Company>,
    user: Option<User>,
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
pub async fn get_money_capitals(
    param: GetParam,
    storage: &State<DatabaseAccountingApi>,
) -> ResponseResult<Vec<models::MoneyCapital>> {
    rocket::debug!("{param:?}");
    let money_capitals = storage
        .get_money_capitals(param.user.map(|u| u.id), param.company.map(|c| c.id))
        .await?;
    Ok(ResponseEnum::ok(
        money_capitals,
        "تم ايجاد رؤؤوس اموال".into(),
    ))
}

#[delete("/<id>")]
pub async fn delete_money_capital(
    id: i64,
    storage: &State<DatabaseAccountingApi>,
    _ug: UGuard,
) -> ResponseResult<()> {
    storage.delete_money_capital(id).await?;
    Ok(ResponseEnum::ok((), "تم مسح رأس المال: {id}".into()))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("money capitals stage", |rocket| async {
        rocket.mount(
            "/api/money_capitals",
            routes![get_money_capitals, delete_money_capital],
        )
    })
}
