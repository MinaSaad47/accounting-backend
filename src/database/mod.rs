pub mod models;
pub mod rows;
use chrono::Utc;
pub use models::*;
pub use rows::*;
use sqlx::pool::PoolOptions;

use crate::accounting_api::{self, AcountingApi};
use rocket::futures::future;
use rocket::{async_trait, fairing::AdHoc};
use sqlx::{Pool, Postgres};
use std::{env, mem};

pub type DB = Postgres;

#[derive(Debug)]
pub struct DatabaseAccountingApi {
    pub db: Pool<DB>,
}

impl DatabaseAccountingApi {
    async fn connect(url: &str) -> sqlx::Result<Self> {
        Ok(DatabaseAccountingApi {
            db: PoolOptions::new().max_connections(100).connect(url).await?,
        })
    }
}

impl From<sqlx::Error> for accounting_api::Error {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => accounting_api::Error::ObjectNotFound,
            _ => accounting_api::Error::Other,
        }
    }
}

#[async_trait]
impl AcountingApi for DatabaseAccountingApi {
    type Company = models::Company;
    type User = models::User;

    async fn create_company(
        &self,
        c: &Self::Company,
    ) -> Result<Self::Company, accounting_api::Error> {
        let mut transaction = self.db.begin().await?;

        let funder = &c.funders[0];
        let company = &c.company;
        let company: rows::Company = sqlx::query_as(
            r#"
            INSERT INTO 
                companies (
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
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING
                *
        "#,
        )
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
        .fetch_one(&mut transaction)
        .await?;
        let funders: Vec<rows::Funder> = sqlx::query_as(
            r#"
            INSERT INTO funders (name, company_id)
            VALUES ($1, $2)
            RETURNING *
        "#,
        )
        .bind(&funder.name)
        .bind(&company.id)
        .fetch_all(&mut transaction)
        .await?;

        transaction.commit().await?;

        Ok(models::Company {
            company,
            funders,
            money_capitals: None,
        })
    }

    async fn update_company(
        &self,
        c: &mut Self::Company,
    ) -> Result<Self::Company, accounting_api::Error> {
        let mut transaction = self.db.begin().await?;

        let company = &c.company;
        let mut funders = mem::take(&mut c.funders);
        let mut money_capitals = mem::take(&mut c.money_capitals);

        let company = sqlx::query_as!(
            rows::Company,
            r#"
                UPDATE
                    companies
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
                WHERE
                    id = $22
                RETURNING
                    id AS "id: _",
                    commercial_feature,
                    is_working,
                    legal_entity,
                    file_number,
                    register_number,
                    start_date,
                    stop_date,
                    general_tax_mission,
                    value_tax_mission,
                    activity_nature,
                    activity_location,
                    accounts,
                    joining_date,
                    natural_id,
                    money_capital,
                    record_side,
                    record_number,
                    user_name,
                    passport,
                    verification_code,
                    email
            "#,
            &company.commercial_feature,
            &company.is_working,
            &company.legal_entity,
            &company.file_number as _,
            &company.register_number,
            &company.start_date,
            &company.stop_date as _,
            &company.general_tax_mission,
            &company.value_tax_mission as _,
            &company.activity_nature,
            &company.activity_location,
            &company.accounts,
            &company.joining_date as _,
            &company.natural_id as _,
            &company.money_capital as _,
            &company.record_side as _,
            &company.record_number as _,
            &company.user_name,
            &company.passport as _,
            &company.verification_code as _,
            &company.email,
            &company.id as _
        )
        .fetch_one(&mut transaction)
        .await?;

        for funder in funders.iter_mut() {
            let opt_funder = sqlx::query_as!(
                rows::Funder,
                r#"
                    UPDATE funders
                    SET
                        name = $2
                    WHERE
                        id = $1
                    RETURNING
                        id AS "id: _", name, company_id AS "company_id!: _"
                "#,
                &funder.id as _,
                &funder.name,
            )
            .fetch_optional(&mut transaction)
            .await?;
            if opt_funder.is_none() {
                *funder = sqlx::query_as!(
                    rows::Funder,
                    r#"
                    INSERT INTO
                        funders (
                            name, company_id
                        )
                    VALUES (
                        $1, $2
                    )
                    RETURNING
                        id AS "id: _", name, company_id AS "company_id!: _"
                "#,
                    &funder.name,
                    &company.id as _,
                )
                .fetch_one(&mut transaction)
                .await?;
            }
        }

        for money_capital in money_capitals.iter_mut().flatten() {
            let opt_money_capital = sqlx::query_as!(
                rows::MoneyCapital,
                r#"
                    UPDATE money_capitals
                    SET
                        value = $2,
                        time = $3
                    WHERE
                        id = $1
                    RETURNING
                        id AS "id: _", value, time, company_id AS "company_id!: _"
                "#,
                &money_capital.id as _,
                &money_capital.value,
                &money_capital.time,
            )
            .fetch_optional(&mut transaction)
            .await?;
            if opt_money_capital.is_none() {
                *money_capital = sqlx::query_as!(
                    rows::MoneyCapital,
                    r#"
                        INSERT INTO
                            money_capitals (
                                value, time, company_id
                            )
                        VALUES (
                            $1, $2, $3
                        )
                        RETURNING
                            id AS "id: _", value, time, company_id AS "company_id!: _"
                    "#,
                    &money_capital.value,
                    Utc::now(),
                    &company.id as _,
                )
                .fetch_one(&mut transaction)
                .await?;
            }
        }

        transaction.commit().await?;
        Ok(models::Company {
            company,
            funders,
            money_capitals,
        })
    }

    async fn search_company(&self, s: &str) -> Result<Vec<Self::Company>, accounting_api::Error> {
        let companies = sqlx::query_as!(
            rows::Company,
            r#"
                SELECT
                    companies.id AS "id: _",
                    commercial_feature,
                    is_working,
                    legal_entity,
                    file_number,
                    register_number,
                    start_date,
                    stop_date,
                    general_tax_mission,
                    value_tax_mission,
                    activity_nature,
                    activity_location,
                    accounts,
                    joining_date,
                    natural_id,
                    money_capital,
                    record_side,
                    record_number,
                    user_name,
                    passport,
                    verification_code,
                    email
                FROM 
                    companies
                JOIN 
                    funders
                ON 
                    companies.id = funders.company_id
                WHERE 
                    companies.id::TEXT LIKE ('%' || $1 || '%') OR funders.name LIKE ('%' || $1 || '%') 
            "#,
            s,
        )
        .fetch_all(&self.db)
        .await?;

        if companies.is_empty() {
            return Err(accounting_api::Error::ObjectNotFound);
        }

        let companies: Vec<models::Company> =
            future::join_all(companies.into_iter().map(|c| async {
                let funders = sqlx::query_as!(
                    rows::Funder,
                    r#"
                        SELECT
                            id AS "id: _", name , company_id AS "company_id!: _"
                        FROM
                            funders
                        WHERE
                            company_id = $1
                    "#,
                    &c.id.expect("queryed_companies must have id")
                )
                .fetch_all(&self.db)
                .await
                .expect("every company must have at least one funder");
                let money_capitals = sqlx::query_as!(
                    rows::MoneyCapital,
                    r#"
                        SELECT
                            id AS "id: _", value, time, company_id AS "company_id!: _"
                        FROM
                            money_capitals
                        WHERE
                            company_id = $1
                    "#,
                    &c.id.expect("queryed_companies must have id")
                )
                .fetch_all(&self.db)
                .await
                .ok();
                models::Company {
                    company: c,
                    funders,
                    money_capitals,
                }
            }))
            .await;
        Ok(companies)
    }

    async fn pay_company(
        &self,
        _c: &Self::Company,
        _v: f64,
    ) -> Result<Self::Company, accounting_api::Error> {
        unimplemented!()
    }

    async fn create_user(&self, _c: &Self::User) -> Result<Self::User, accounting_api::Error> {
        unimplemented!()
    }
    async fn update_user(&self, _c: &Self::User) -> Result<Self::User, accounting_api::Error> {
        unimplemented!()
    }
    async fn search_user(&self, _s: &str) -> Result<Vec<Self::User>, accounting_api::Error> {
        unimplemented!()
    }
    async fn pay_user(
        &self,
        _c: &Self::User,
        _v: f64,
    ) -> Result<Self::User, accounting_api::Error> {
        unimplemented!()
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("database stage", |rocket| async {
        let storage = DatabaseAccountingApi::connect(
            &env::var("DATABASE_URL").expect("`DATABASE_URL` must be set"),
        )
        .await
        .expect("database connection");
        sqlx::migrate!("db/migrations")
            .run(&storage.db)
            .await
            .expect("migrations run");
        rocket.manage(storage)
    })
}
