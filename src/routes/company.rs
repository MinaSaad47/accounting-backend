use rocket::{
    get, post,
    response::status::{Created, NotFound},
    serde::json::Json,
    State,
};

use crate::{
    db::{rows, Storage},
    models,
    types::{response::Response, uuid::Uuid},
};

#[post("/company", format = "application/json", data = "<company>")]
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

    let funders: Vec<rows::Funder> = sqlx::query_as(
        r#"
            INSERT INTO funders (name, company_id)
            VALUES ($1, $2)
            RETURNING *
        "#,
    )
    .bind(&funder.name)
    .bind(&uuid)
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
            funders
        }),
    })))
}

#[get("/company?<search>")]
pub async fn get(
    search: &str,
    storage: &State<Storage>,
) -> Result<Json<Vec<rows::Company>>, NotFound<String>> {
    let companies: Vec<rows::Company> = sqlx::query_as(&format!(
        r#"
                SELECT *
                FROM companies
            "#,
    ))
    .fetch_all(&storage.db)
    .await
    .map_err(|e| NotFound(format!("{e}")))?;
    Ok(Json(
        companies
            .into_iter()
            .filter(|c| c.user_name.contains(search))
            .collect(),
    ))
}
