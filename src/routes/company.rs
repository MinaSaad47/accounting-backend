use rocket::{
    fairing::AdHoc,
    futures::future,
    get, post, put,
    response::status::{BadRequest, Created, NotFound},
    routes,
    serde::json::Json,
    State,
};

use std::mem;

use crate::{
    db::{rows, Storage},
    models,
    types::{response::Response, uuid::Uuid},
};

#[post("/", format = "application/json", data = "<company>")]
pub async fn post(
    company: Json<models::Company>,
    storage: &State<Storage>,
) -> Result<Created<Json<Response<models::Company>>>, Json<Response<models::Company>>> {
    let funder = &company.funders[0];
    let company = &company.company;
    let uuid = Uuid::new();
    let company: rows::Company = sqlx::query_as(
        r#"
            INSERT INTO companies (
                id,
                commercial_feature,
                is_working,
                legal_entity,
                register_number,
                start_date,
                general_tax_mission,
                activity_nature,
                activity_location,
                accounts,
                record_number,
                user_name,
                email
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING *
        "#,
    )
    .bind(&uuid)
    .bind(&company.commercial_feature)
    .bind(&company.is_working)
    .bind(&company.legal_entity)
    .bind(&company.register_number)
    .bind(&&company.start_date)
    .bind(&company.general_tax_mission)
    .bind(&company.activity_nature)
    .bind(&company.activity_location)
    .bind(&company.accounts)
    .bind(&company.record_number)
    .bind(&company.user_name)
    .bind(&company.email)
    .fetch_one(&storage.db)
    .await
    .map_err(|e| {
        Json(Response {
            status: false,
            message: format!("{e}"),
            data: None,
        })
    })?;

    let uuid = Uuid::new();
    let funders: Vec<rows::Funder> = sqlx::query_as(
        r#"
            INSERT INTO funders (id, name, company_id)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
    )
    .bind(&uuid)
    .bind(&funder.name)
    .bind(&company.id)
    .fetch_all(&storage.db)
    .await
    .map_err(|e| {
        Json(Response {
            status: false,
            message: format!("{e}"),
            data: None,
        })
    })?;

    Ok(Created::new("").body(Json(Response {
        status: true,
        message: "تم حفظ الشركة بنجاح".to_string(),
        data: Some(models::Company {
            company,
            funders,
            money_capitals: None,
        }),
    })))
}

#[get("/?<search>")]
pub async fn get(
    search: &str,
    storage: &State<Storage>,
) -> Result<Json<Response<Vec<models::Company>>>, NotFound<Json<Response<Vec<models::Company>>>>> {
    let companies: Vec<rows::Company> = sqlx::query_as(&format!(
        r#"
            SELECT *
            FROM companies
        "#,
    ))
    .fetch_all(&storage.db)
    .await
    .map_err(|e| {
        NotFound(Json(Response {
            status: false,
            message: format!("{e}"),
            data: None,
        }))
    })?;

    let companies = companies.into_iter().map(|company| async {
        let funders: Vec<rows::Funder> = sqlx::query_as(
            r#"
                SELECT *
                FROM funders
                WHERE company_id = $1
            "#,
        )
        .bind(&company.id)
        .fetch_all(&storage.db)
        .await
        .expect("a company must have at least one funder");
        let money_capitals: Option<Vec<rows::MoneyCapital>> = sqlx::query_as(
            r#"
                SELECT *
                FROM money_capitals
                WHERE company_id = $1
            "#,
        )
        .bind(&company.id)
        .fetch_all(&storage.db)
        .await
        .ok();
        models::Company {
            company,
            funders,
            money_capitals,
        }
    });

    let companies = future::join_all(companies).await;

    let companies: Vec<models::Company> = companies
        .into_iter()
        .filter(|company| {
            company
                .company
                .id
                .as_ref()
                .expect("id not null constraint")
                .to_string()
                .contains(search)
                || company
                    .funders
                    .iter()
                    .any(|funder| funder.name.contains(search))
        })
        .collect();

    if companies.is_empty() {
        Err(NotFound(Json(Response {
            status: false,
            message: "لم يتم اجاد شركات".to_string(),
            data: None,
        })))
    } else {
        Ok(Json(Response {
            status: true,
            message: "تم اجاد شركات".to_string(),
            data: Some(companies),
        }))
    }
}

#[put("/", format = "application/json", data = "<company>")]
pub async fn put(
    mut company: Json<models::Company>,
    storage: &State<Storage>,
) -> Result<Created<Json<Response<models::Company>>>, BadRequest<Json<Response<models::Company>>>> {
    let mut funders = mem::take(&mut company.funders);
    let mut money_capitals = mem::take(&mut company.money_capitals);
    let company = &company.company;
    let company: rows::Company = sqlx::query_as(
        r#"
            UPDATE companies
            SET
                commercial_feature = $1,
                is_working = $2,
                legal_entity = $3,
                file_number = $4,
                register_number = $5,
                start_date = $6,
                stop_date = $7,
                general_tax_mission = $8,
                value_tax_mission = $9,
                activity_nature = $10,
                activity_location = $11,
                accounts = $12,
                joining_date = $13,
                natural_id = $14,
                money_capital = $15,
                record_side = $16,
                record_number = $17,
                user_name = $18,
                passport = $19,
                verification_code = $20,
                email = $21
            WHERE id = $22
            RETURNING *
        "#,
    )
    .bind(&company.commercial_feature)
    .bind(&company.is_working)
    .bind(&company.legal_entity)
    .bind(&company.file_number)
    .bind(&company.register_number)
    .bind(&company.start_date)
    .bind(&company.stop_date)
    .bind(&company.general_tax_mission)
    .bind(&company.value_tax_mission)
    .bind(&company.activity_nature)
    .bind(&company.activity_location)
    .bind(&company.accounts)
    .bind(&company.joining_date)
    .bind(&company.natural_id)
    .bind(&company.money_capital)
    .bind(&company.record_side)
    .bind(&company.record_number)
    .bind(&company.user_name)
    .bind(&company.passport)
    .bind(&company.verification_code)
    .bind(&company.email)
    .bind(&company.id)
    .fetch_one(&storage.db)
    .await
    .map_err(|e| {
        BadRequest(Some(Json(Response {
            status: false,
            message: format!("{e}"),
            data: None,
        })))
    })?;

    for funder in funders.iter_mut() {
        funder.id = Some(funder.id.take().unwrap_or(Uuid::new()));
        *funder = sqlx::query_as(
            r#"
                INSERT OR REPLACE INTO funders (
                    id, name, company_id
                )
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
        )
        .bind(&funder.id)
        .bind(&funder.name)
        .bind(&company.id)
        .fetch_one(&storage.db)
        .await
        .map_err(|e| {
            BadRequest(Some(Json(Response {
                status: false,
                message: format!("{e}"),
                data: None,
            })))
        })?
    }

    for money_capital in money_capitals.iter_mut().flatten() {
        money_capital.id = Some(money_capital.id.take().unwrap_or(Uuid::new()));
        *money_capital = sqlx::query_as(
            r#"
                INSERT OR REPLACE INTO money_capitals (
                    id, value, time, company_id
                )
                VALUES ($1, $2, $3, $4)
                RETURNING *
            "#,
        )
        .bind(&money_capital.id)
        .bind(&money_capital.value)
        .bind(&money_capital.time)
        .bind(&company.id)
        .fetch_one(&storage.db)
        .await
        .map_err(|e| {
            BadRequest(Some(Json(Response {
                status: false,
                message: format!("{e}"),
                data: None,
            })))
        })?
    }
    Ok(Created::new("").body(Json(Response {
        status: true,
        message: "تم حفظ الشركة بنجاح".to_string(),
        data: Some(models::Company {
            company,
            funders,
            money_capitals,
        }),
    })))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("companies stage", |rocket| async {
        rocket.mount("/api/company", routes![post, put, get])
    })
}
