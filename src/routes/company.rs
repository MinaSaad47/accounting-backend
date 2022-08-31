use rocket::{delete, fairing::AdHoc, get, post, put, routes, serde::json::Json, State};

use crate::{
    accounting_api::AcountingApi,
    auth::{AGuard, UGuard},
    database::{models, rows, DatabaseAccountingApi},
    types::response::{ResponseEnum, ResponseResult},
};

#[post("/", format = "application/json", data = "<company>")]
pub async fn create_company(
    company: Json<models::Company>,
    storage: &State<DatabaseAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<models::Company> {
    rocket::trace!("{company:#?}");
    let company = storage.create_company(&company.into_inner()).await?;
    Ok(ResponseEnum::created(company, "تم انشاء شركة جديدة".into()))
}

#[get("/?<search>")]
pub async fn search_company_admin(
    search: &str,
    storage: &State<DatabaseAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<Vec<models::Company>> {
    rocket::trace!("{search:#?}");
    let companies = storage.search_company(search).await?;
    Ok(ResponseEnum::ok(companies, "تم العثور علي شركات".into()))
}

#[get("/?<search>", rank = 2)]
pub async fn search_company_user(
    search: &str,
    storage: &State<DatabaseAccountingApi>,
    _ug: UGuard,
) -> ResponseResult<Vec<models::Company>> {
    rocket::trace!("{search:#?}");
    let companies = storage.search_company(search).await?;
    Ok(ResponseEnum::ok(companies, "تم العثور علي شركات".into()))
}

#[put("/", format = "application/json", data = "<company>")]
pub async fn update_company(
    company: Json<models::Company>,
    storage: &State<DatabaseAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<models::Company> {
    rocket::trace!("{company:#?}");
    let compannies = storage.update_company(&mut company.into_inner()).await?;
    Ok(ResponseEnum::ok(compannies, "تم خفظ الشركة بنجاح".into()))
}

#[post(
    "/<company_id>/expenses",
    format = "application/json",
    data = "<expense>"
)]
pub async fn create_expense(
    company_id: i64,
    expense: Json<rows::Expense>,
    storage: &State<DatabaseAccountingApi>,
    ug: UGuard,
) -> ResponseResult<models::Expense> {
    let expense = storage
        .create_expense(
            ug.0,
            company_id,
            expense.value,
            &expense.description,
        )
        .await?;

    Ok(ResponseEnum::created(expense, "تم اضافة رأس مال".into()))
}

#[post(
    "/<company_id>/incomes",
    format = "application/json",
    data = "<income>"
)]
pub async fn create_income(
    company_id: i64,
    income: Json<rows::Income>,
    storage: &State<DatabaseAccountingApi>,
    ag: AGuard,
) -> ResponseResult<models::Expense> {
    let expense = storage
        .create_expense(
            ag.0,
            company_id,
            income.value,
            &income.description,
        )
        .await?;

    Ok(ResponseEnum::created(expense, "تم اضافة رأس مال".into()))
}

#[delete("/<id>")]
pub async fn delete_company(
    id: i64,
    storage: &State<DatabaseAccountingApi>,
    _ag: AGuard,
) -> ResponseResult<()> {
    storage.delete_company(id).await?;
    Ok(ResponseEnum::ok((), "تم حذف الشركة".into()))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("companies stage", |rocket| async {
        rocket.mount(
            "/api/company",
            routes![
                create_company,
                update_company,
                search_company_admin,
                search_company_user,
                create_expense,
                create_income,
                delete_company,
            ],
        )
    })
}
