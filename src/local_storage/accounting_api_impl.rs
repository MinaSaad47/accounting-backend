use std::{io, mem};

use crate::accounting_api::{self, AcountingApi};
use rocket::{async_trait, fs::TempFile, futures::future};

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

impl From<io::Error> for accounting_api::Error {
    fn from(error: io::Error) -> Self {
        rocket::error!("[FileSystem] {error:#?}");
        Self::Other(error.to_string().into())
    }
}

#[async_trait]
impl AcountingApi for super::LocalStorageAccountingApi {
    type Company = models::Company;
    type User = models::User;
    type Expense = models::Expense;
    type Income = models::Income;
    type Document = models::Document;
    type Funder = models::Funder;
    type Error = accounting_api::Error;

    async fn create_company(
        &self,
        c: &Self::Company,
    ) -> Result<Self::Company, accounting_api::Error> {
        let mut transaction = self.db.begin().await?;

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
        .bind(&c.as_ref().commercial_feature)
        .bind(&c.as_ref().is_working)
        .bind(&c.as_ref().legal_entity)
        .bind(&c.as_ref().register_number)
        .bind(&c.as_ref().start_date)
        .bind(&c.as_ref().general_tax_mission)
        .bind(&c.as_ref().activity_nature)
        .bind(&c.as_ref().activity_location)
        .bind(&c.as_ref().accounts)
        .bind(&c.as_ref().record_number)
        .bind(&c.as_ref().user_name)
        .bind(&c.as_ref().email)
        .fetch_one(&mut transaction)
        .await?;

        transaction.commit().await?;

        Ok(company.into())
    }

    async fn update_company(
        &self,
        c: &mut Self::Company,
    ) -> Result<Self::Company, accounting_api::Error> {
        let mut transaction = self.db.begin().await?;

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
                    record_side = $15,
                    record_number = $16,
                    user_name = $17,
                    passport = $18,
                    verification_code = $19,
                    email = $20
                WHERE
                    id = $21
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
                    record_side,
                    record_number,
                    user_name,
                    passport,
                    verification_code,
                    email
            "#,
            &c.as_ref().commercial_feature,
            &c.as_ref().is_working,
            &c.as_ref().legal_entity,
            &c.as_ref().file_number as _,
            &c.as_ref().register_number,
            &c.as_ref().start_date,
            &c.as_ref().stop_date as _,
            &c.as_ref().general_tax_mission,
            &c.as_ref().value_tax_mission as _,
            &c.as_ref().activity_nature,
            &c.as_ref().activity_location,
            &c.as_ref().accounts,
            &c.as_ref().joining_date as _,
            &c.as_ref().natural_id as _,
            &c.as_ref().record_side as _,
            &c.as_ref().record_number as _,
            &c.as_ref().user_name,
            &c.as_ref().passport as _,
            &c.as_ref().verification_code as _,
            &c.as_ref().email,
            &c.as_ref().id as _
        )
        .fetch_one(&mut transaction)
        .await?;
        transaction.commit().await?;
        Ok(company.into())
    }

    async fn search_company(&self, s: &str) -> Result<Vec<Self::Company>, accounting_api::Error> {
        let companies = sqlx::query_as!(
            rows::Company,
            r#"
                SELECT DISTINCT ON (companies.id)
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
                    record_side,
                    record_number,
                    user_name,
                    passport,
                    verification_code,
                    email
                FROM 
                    companies
                LEFT JOIN 
                    funders
                ON 
                    companies.id = funders.company_id
                WHERE 
                    companies.id::TEXT ILIKE ('%' || $1 || '%') OR
                    funders.name ILIKE ('%' || $1 || '%') OR
                    companies.commercial_feature ILIKE ('%' || $1 || '%')
            "#,
            s,
        )
        .fetch_all(&self.db)
        .await?;

        if companies.is_empty() {
            return Err(Self::Error::ObjectNotFound);
        }

        Ok(companies.into_iter().map(|c| c.into()).collect())
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
                    users (name, password, is_admin, value)
                VALUES
                    ($1, $2, $3, 0)
                RETURNING
                    id AS "id: _", name, password, is_admin, value
            "#,
            &user.name,
            &user.password,
            &user.is_admin,
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

    async fn get_expenses(
        &self,
        user_id: Option<i64>,
        company_id: Option<i64>,
    ) -> Result<Vec<Self::Expense>, Self::Error> {
        let expenses = sqlx::query_as!(
            rows::Expense,
            r#"
                SELECT
                    id AS "id: _",
                    value,
                    description,
                    time AS "time?: _",
                    user_id AS "user_id!: _",
                    company_id AS "company_id!: _"
                FROM
                    expenses
                WHERE
                    (user_id = $1 OR $1 IS NULL) AND (company_id = $2 OR $2 IS NULL)
            "#,
            user_id,
            company_id,
        )
        .fetch_all(&self.db)
        .await?;

        let expenses = future::join_all(expenses.into_iter().map(|expense| async {
            let user = sqlx::query!(
                r#"
                    SELECT
                        name
                    FROM
                        users
                    WHERE
                        id = $1
                "#,
                expense.user_id,
            )
            .fetch_one(&self.db)
            .await
            .expect("user name from expense")
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
                expense.company_id,
            )
            .fetch_one(&self.db)
            .await
            .expect("company commercial_feature from expense")
            .commercial_feature;

            models::Expense {
                expense,
                user,
                company,
            }
        }))
        .await;

        Ok(expenses)
    }

    async fn create_expense(
        &self,
        user_id: i64,
        company_id: i64,
        value: f64,
        description: &str,
    ) -> Result<Self::Expense, Self::Error> {
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

        let expense = sqlx::query_as!(
            rows::Expense,
            r#"
                INSERT INTO
                    expenses (user_id, company_id, value, description)
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
        Ok(Self::Expense {
            user: user.name,
            company: company.commercial_feature,
            expense,
        })
    }
    async fn delete_expense(&self, id: i64) -> Result<(), Self::Error> {
        let mut transaction = self.db.begin().await?;
        let result = sqlx::query!(
            r#"
                DELETE FROM
                    expenses
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
    async fn get_incomes(
        &self,
        admin_id: Option<i64>,
        company_id: Option<i64>,
    ) -> Result<Vec<Self::Income>, Self::Error> {
        let incomes = sqlx::query_as!(
            rows::Income,
            r#"
                SELECT
                    id AS "id: _",
                    value,
                    description,
                    time AS "time?: _",
                    admin_id AS "admin_id!: _",
                    company_id AS "company_id!: _"
                FROM
                    incomes
                WHERE
                    (admin_id = $1 OR $1 IS NULL) AND (company_id = $2 OR $2 IS NULL)
            "#,
            admin_id,
            company_id,
        )
        .fetch_all(&self.db)
        .await?;

        let incomes = future::join_all(incomes.into_iter().map(|income| async {
            let admin = sqlx::query!(
                r#"
                    SELECT
                        name
                    FROM
                        users
                    WHERE
                        id = $1
                "#,
                income.admin_id,
            )
            .fetch_one(&self.db)
            .await
            .expect("user name from income")
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
                income.company_id,
            )
            .fetch_one(&self.db)
            .await
            .expect("company commercial_feature from income")
            .commercial_feature;

            models::Income {
                income,
                admin,
                company,
            }
        }))
        .await;

        Ok(incomes)
    }

    async fn create_income(
        &self,
        admin_id: i64,
        company_id: i64,
        value: f64,
        description: &str,
    ) -> Result<Self::Income, Self::Error> {
        let admin = sqlx::query!(
            r#"
                SELECT
                    id, name
                FROM
                    users
                WHERE
                    id = $1
            "#,
            &admin_id
        )
        .fetch_one(&self.db)
        .await?;

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

        let income = sqlx::query_as!(
            rows::Income,
            r#"
                INSERT INTO
                    incomes (admin_id, company_id, value, description)
                VALUES
                    ($1, $2, $3, $4)
                RETURNING
                    id AS "id?: _",
                    admin_id AS "admin_id!: _",
                    company_id AS "company_id!: _",
                    value,
                    description,
                    time AS "time?: _"
            "#,
            admin.id,
            company.id,
            value,
            description,
        )
        .fetch_one(&mut transaction)
        .await?;

        transaction.commit().await?;
        Ok(Self::Income {
            admin: admin.name,
            company: company.commercial_feature,
            income,
        })
    }
    async fn delete_income(&self, id: i64) -> Result<(), Self::Error> {
        let mut transaction = self.db.begin().await?;
        sqlx::query!(
            r#"
                DELETE FROM
                    incomes
                WHERE
                    id = $1
            "#,
            id
        )
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn create_document(
        &self,
        company_id: i64,
        file: &mut TempFile<'_>,
    ) -> Result<Self::Document, Self::Error> {
        let mut transaction = self.db.begin().await?;

        let file_name = file
            .raw_name()
            .map(|n| n.dangerous_unsafe_unsanitized_raw().as_str())
            .ok_or(Self::Error::Other("اسم ملف غير امن".into()))?;

        let document = sqlx::query_as!(
            rows::Document,
            r#"
                INSERT INTO
                    documents (name, company_id)
                VALUES
                    ($1, $2)
                RETURNING 
                    *
            "#,
            file_name,
            company_id,
        )
        .fetch_one(&mut transaction)
        .await?;

        self.fs
            .write()
            .await
            .save(document.to_path_buf(), file)
            .await?;

        transaction.commit().await?;
        Ok(document.into())
    }

    async fn get_documents(&self, company_id: i64) -> Result<Vec<Self::Document>, Self::Error> {
        let docuemnts = sqlx::query_as!(
            rows::Document,
            r#"
                SELECT
                    *
                FROM
                    documents
                WHERE
                    company_id = $1
            "#,
            company_id,
        )
        .fetch_all(&self.db)
        .await?;

        let documents: Vec<models::Document> = docuemnts
            .into_iter()
            .map(|doc| models::Document::from(doc))
            .collect();

        Ok(documents)
    }
    async fn delete_document(&self, id: i64) -> Result<(), Self::Error> {
        let mut transaction = self.db.begin().await?;
        let document = sqlx::query_as!(
            rows::Document,
            r#"
                DELETE FROM
                    documents
                WHERE
                    id = $1
                RETURNING
                    *
            "#,
            id,
        )
        .fetch_one(&mut transaction)
        .await?;

        self.fs.write().await.delete(document.to_path_buf()).await?;

        transaction.commit().await?;
        Ok(())
    }

    async fn create_funder(
        &self,
        company_id: i64,
        f: &Self::Funder,
    ) -> Result<Self::Funder, Self::Error> {
        let mut transaction = self.db.begin().await?;

        let funder = sqlx::query_as!(
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
                    id AS "id?: _",
                    name,
                    company_id AS "company_id!: _"
            "#,
            f.as_ref().name as _,
            company_id as _,
        )
        .fetch_one(&mut transaction)
        .await?;
        transaction.commit().await?;
        Ok(funder.into())
    }
    async fn get_funders(&self, company_id: i64) -> Result<Vec<Self::Funder>, Self::Error> {
        let funders = sqlx::query_as!(
            rows::Funder,
            r#"
                SELECT
                    id AS "id?: _",
                    name,
                    company_id AS "company_id!: _"
                FROM
                    funders
                WHERE
                    company_id = $1
            "#,
            company_id,
        )
        .fetch_all(&self.db)
        .await?;

        Ok(funders.into_iter().map(|f| f.into()).collect())
    }
    async fn delete_funder(&self, id: i64) -> Result<(), Self::Error> {
        let mut transaction = self.db.begin().await?;

        sqlx::query!(
            r#"
                DELETE FROM
                    funders
                WHERE
                    id = $1
            "#,
            id,
        )
        .execute(&mut transaction)
        .await?;

        transaction.commit().await?;
        Ok(())
    }
}
