use rocket::{
    get, post,
    response::status::{Created, NotFound, Unauthorized},
    serde::{json::Json, Deserialize, Serialize},
    State,
};

use crate::{
    db::Storage,
    models::{company::Company, funder::Funder},
    types::uuid::Uuid,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateData {
    pub funder: Funder,
    pub company: Company,
}

#[post("/company", format = "application/json", data = "<data>")]
pub async fn post(
    data: Json<CreateData>,
    storage: &State<Storage>,
) -> Result<Created<Json<Company>>, Unauthorized<String>> {
    let data = data.into_inner();
    let funder = &data.funder;
    let company = &data.company;
    let uuid = Uuid::new();
    let created_company = sqlx::query_as(
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
    .map_err(|e| Unauthorized(Some(format!("{e}"))))?;

    sqlx::query!(
        r#"
            INSERT INTO funders (name, company_id)
            VALUES ($1, $2)
        "#,
        funder.name,
        uuid,
    )
    .execute(&storage.db)
    .await
    .map_err(|e| Unauthorized(Some(format!("{e}"))))?;

    Ok(Created::new("").body(Json(created_company)))
}

#[get("/company?<search>")]
pub async fn get(
    search: &str,
    storage: &State<Storage>,
) -> Result<Json<Vec<Company>>, NotFound<String>> {
    let companies: Vec<Company> = sqlx::query_as(&format!(
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
