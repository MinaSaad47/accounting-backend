use std::mem;

use crate::accounting_api::{self, AcountingApi};
use rocket::{async_trait, futures::future};

use chrono::Utc;
use sqlx::postgres::PgDatabaseError;

use super::{models, rows};

impl From<sqlx::Error> for accounting_api::Error {
    fn from(error: sqlx::Error) -> Self {
        rocket::error!("[Database] {error:#?}");
        match error {
            sqlx::Error::RowNotFound => accounting_api::Error::ObjectNotFound,
            sqlx::Error::Database(error) => error
                .try_downcast_ref::<PgDatabaseError>()
                .and_then(|error| error.detail())
                .map(|error| accounting_api::Error::Other(error.to_owned().into()))
                .unwrap_or(accounting_api::Error::Other("غير معروف".into())),
            _ => accounting_api::Error::Other("غير معروف".into()),
        }
    }
}

#[async_trait]
impl AcountingApi for super::DatabaseAccountingApi {
    type Company = models::Company;
    type User = models::User;
    type MoneyCapital = models::MoneyCapital;
    type Error = accounting_api::Error;

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
                        value = $2
                    WHERE
                        id = $1
                    RETURNING
                        id AS "id: _",
                        value,
                        description,
                        time AS "time?: _",
                        user_id AS "user_id!: _",
                        company_id AS "company_id!: _"
                "#,
                &money_capital.id as _,
                &money_capital.value,
            )
            .fetch_optional(&mut transaction)
            .await?;
            if opt_money_capital.is_none() {
                *money_capital = sqlx::query_as!(
                    rows::MoneyCapital,
                    r#"
                        INSERT INTO
                            money_capitals (
                                value, description, time, company_id
                            )
                        VALUES (
                            $1, $2, $3, $4
                        )
                        RETURNING
                        id AS "id: _",
                        value,
                        description,
                        time AS "time?: _",
                        user_id AS "user_id!: _",
                        company_id AS "company_id!: _"
                    "#,
                    &money_capital.value,
                    &money_capital.description,
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
            return Err(Self::Error::ObjectNotFound);
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
                            id AS "id: _",
                            value,
                            description,
                            time AS "time?: _",
                            user_id AS "user_id!: _",
                            company_id AS "company_id!: _"
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

    async fn pay_company(&self, _c: &Self::Company, _v: f64) -> Result<Self::Company, Self::Error> {
        unimplemented!()
    }

    async fn create_user(&self, c: &Self::User) -> Result<Self::User, Self::Error> {
        let mut transaction = self.db.begin().await?;

        let user = &c.user;

        let user = sqlx::query_as!(
            rows::User,
            r#"
                INSERT INTO
                    users (name, password, value)
                VALUES
                    ($1, $2, 0)
                RETURNING
                    id AS "id: _", name, password, is_admin, value
            "#,
            &user.name,
            &user.password,
        )
        .fetch_one(&mut transaction)
        .await?;

        transaction.commit().await?;
        Ok(models::User { user })
    }
    async fn update_user(&self, c: &Self::User) -> Result<Self::User, Self::Error> {
        let mut transaction = self.db.begin().await?;
        let user = &c.user;

        let user = sqlx::query_as!(
            rows::User,
            r#"
                UPDATE
                    users
                SET
                    name = $2,
                    password = $3
                WHERE
                    id = $1
                RETURNING
                    id AS "id: _", name, password, is_admin, value
            "#,
            &user.id as _,
            &user.name,
            &user.password,
        )
        .fetch_one(&mut transaction)
        .await?;

        transaction.commit().await?;
        Ok(models::User { user })
    }
    async fn get_users(&self) -> Result<Vec<Self::User>, Self::Error> {
        let users = sqlx::query_as!(
            rows::User,
            r#"
                SELECT
                    id AS "id: _",
                    name,
                    password,
                    is_admin,
                    value
                FROM
                    users
            "#,
        )
        .fetch_all(&self.db)
        .await?;

        Ok(users
            .into_iter()
            .map(|user| models::User { user })
            .collect())
    }
    async fn pay_user(&self, id: i64, v: f64) -> Result<Self::User, Self::Error> {
        let mut transaction = self.db.begin().await?;

        let user = sqlx::query_as!(
            rows::User,
            r#"
                UPDATE
                    users
                SET
                    value = $2
                WHERE
                    id = $1
                RETURNING
                    id AS "id: _", name, password, is_admin, value
            "#,
            id as _,
            v,
        )
        .fetch_one(&mut transaction)
        .await?;

        transaction.commit().await?;
        Ok(models::User { user })
    }

    async fn login_user(&self, u: &Self::User) -> Result<Self::User, Self::Error> {
        let user = &u.user;
        let user = sqlx::query_as!(
            rows::User,
            r#"
                SELECT
                    id AS "id: _",
                    name,
                    password,
                    is_admin,
                    value
                FROM
                    users
                WHERE
                    name = $1 AND password = $2
            "#,
            &user.name,
            &user.password,
        )
        .fetch_one(&self.db)
        .await?;
        Ok(models::User { user })
    }

    async fn get_money_capitals(
        &self,
        user_id: Option<i64>,
        company_id: Option<i64>,
    ) -> Result<Vec<Self::MoneyCapital>, Self::Error> {
        let money_capitals = sqlx::query_as!(
            rows::MoneyCapital,
            r#"
                SELECT
                    id AS "id: _",
                    value,
                    description,
                    time AS "time?: _",
                    user_id AS "user_id!: _",
                    company_id AS "company_id!: _"
                FROM
                    money_capitals
                WHERE
                    (user_id = $1 OR $1 IS NULL) AND (company_id = $2 OR $2 IS NULL)
            "#,
            user_id,
            company_id,
        )
        .fetch_all(&self.db)
        .await?;

        let money_capitals =
            future::join_all(money_capitals.into_iter().map(|money_capital| async {
                let user = sqlx::query!(
                    r#"
                    SELECT
                        name
                    FROM
                        users
                    WHERE
                        id = $1
                "#,
                    money_capital.user_id,
                )
                .fetch_one(&self.db)
                .await
                .expect("user name from money_capital")
                .name;

                let company = sqlx::query!(
                    r#"
                    SELECT
                        commercial_feature
                    FROM
                        companies
                    WHERE
                        id = $1
                "#,
                    money_capital.company_id,
                )
                .fetch_one(&self.db)
                .await
                .expect("company commercial_feature from money_capital")
                .commercial_feature;

                models::MoneyCapital {
                    money_capital,
                    user,
                    company,
                }
            }))
            .await;

        Ok(money_capitals)
    }

    async fn create_money_capital(
        &self,
        user_id: i64,
        company_id: i64,
        value: f64,
        description: &str,
    ) -> Result<Self::MoneyCapital, Self::Error> {
        let user = sqlx::query!(
            r#"
                SELECT
                    id, value, name
                FROM
                    users
                WHERE
                    id = $1
            "#,
            &user_id
        )
        .fetch_one(&self.db)
        .await?;

        if value > user.value {
            return Err(Self::Error::NotEnoughUserValue(value, user.value));
        }

        let company = sqlx::query!(
            r#"
                SELECT
                    id, commercial_feature
                FROM
                    companies
                WHERE
                    id = $1
            "#,
            company_id,
        )
        .fetch_one(&self.db)
        .await?;

        let mut transaction = self.db.begin().await?;

        sqlx::query!(
            r#"
                UPDATE
                    users
                SET
                    value = $2
                WHERE id = $1
            "#,
            user.id,
            user.value - value,
        )
        .execute(&mut transaction)
        .await?;

        let money_capital = sqlx::query_as!(
            rows::MoneyCapital,
            r#"
                INSERT INTO
                    money_capitals (user_id, company_id, value, description)
                VALUES
                    ($1, $2, $3, $4)
                RETURNING
                    id AS "id?: _",
                    user_id AS "user_id!: _",
                    company_id AS "company_id!: _",
                    value,
                    description,
                    time AS "time?: _"
            "#,
            user.id,
            company.id,
            value,
            description,
        )
        .fetch_one(&mut transaction)
        .await?;

        transaction.commit().await?;
        Ok(Self::MoneyCapital {
            user: user.name,
            company: company.commercial_feature,
            money_capital,
        })
    }
    async fn delete_money_capital(&self, id: i64) -> Result<(), Self::Error> {
        let mut transaction = self.db.begin().await?;
        let result = sqlx::query!(
            r#"
                DELETE FROM
                    money_capitals
                WHERE
                    id = $1
                RETURNING
                    user_id, value
            "#,
            id
        )
        .fetch_one(&mut transaction)
        .await?;

        sqlx::query!(
            r#"
                UPDATE
                    users
                SET
                    value = value + $2
                WHERE
                    id = $1
            "#,
            result.user_id,
            result.value,
        )
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        Ok(())
    }
    async fn get_user(&self, id: i64) -> Result<Self::User, Self::Error> {
        let user = sqlx::query_as!(
            rows::User,
            r#"
                SELECT
                    id AS "id: _",
                    name,
                    password,
                    value,
                    is_admin
                FROM
                    users
                WHERE
                    id = $1
            "#,
            id,
        )
        .fetch_one(&self.db)
        .await?;
        Ok(models::User { user })
    }

    async fn delete_company(&self, id: i64) -> Result<(), Self::Error> {
        sqlx::query!(
            r#"
                DELETE FROM
                    companies
                WHERE
                    id = $1
            "#,
            id,
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    async fn delete_user(&self, id: i64) -> Result<(), Self::Error> {
        sqlx::query!(
            r#"
                DELETE FROM
                    users
                WHERE
                    id = $1
            "#,
            id,
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }
}
