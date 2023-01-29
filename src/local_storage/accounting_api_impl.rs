use std::{io, path::Path};

use crate::{
    accounting_api::{self, AcountingApi},
    file_system::FileSystemFile,
    local_storage::models::*,
};
use rocket::{async_trait, fs::TempFile};

use sqlx::{postgres::PgDatabaseError, types::Uuid};

use super::models;

impl From<sqlx::Error> for accounting_api::Error {
    fn from(error: sqlx::Error) -> Self {
        rocket::error!("[Database] {error:#?}");
        match error {
            sqlx::Error::RowNotFound => accounting_api::Error::ObjectNotFound,
            sqlx::Error::Database(error) => error
                .try_downcast_ref::<PgDatabaseError>()
                .map(|error| accounting_api::Error::Other(error.message().to_owned().into()))
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
        c: &CreateCompany,
    ) -> Result<Self::Company, accounting_api::Error> {
        let mut transaction = self.db.begin().await?;

        let company = sqlx::query_as!(
            models::Company,
            r#"
                INSERT INTO 
                    companies (
                        owner,
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
                        record_number,
                        username,
                        password,
                        email
                    )
                VALUES
                    ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
                RETURNING
                    *
            "#,
            &c.owner,
            &c.commercial_feature,
            &c.is_working,
            &c.legal_entity as _,
            &c.file_number as _,
            &c.register_number as _,
            &c.start_date as _,
            &c.stop_date as _,
            &c.general_tax_mission as _,
            &c.value_tax_mission as _,
            &c.activity_nature as _,
            &c.activity_location as _,
            &c.record_number as _,
            &c.username as _,
            &c.password as _,
            &c.email as _,
        )
        .fetch_one(&mut transaction)
        .await?;

        transaction.commit().await?;

        Ok(company)
    }

    async fn update_company(
        &self,
        id: Uuid,
        c: &UpdateCompany,
    ) -> Result<Self::Company, accounting_api::Error> {
        let mut transaction = self.db.begin().await?;

        let old_company = sqlx::query!(
            r#"
                SELECT
                    owner, commercial_feature
                FROM
                    companies
                WHERE
                    id = $1
            "#,
            id
        )
        .fetch_one(&self.db)
        .await?;

        let company = sqlx::query_as!(
            models::Company,
            r#"
                UPDATE
                    companies
                SET
                    owner = $1,
                    commercial_feature = $2,
                    is_working = $3,
                    legal_entity = $4,
                    file_number = $5,
                    register_number = $6,
                    start_date = $7,
                    stop_date = $8,
                    general_tax_mission = $9,
                    value_tax_mission = $10,
                    activity_nature = $11,
                    activity_location = $12,
                    record_number = $13,
                    username = $14,
                    password = $15,
                    email = $16
                WHERE
                    id = $17
                RETURNING
                    *
            "#,
            &c.owner,
            &c.commercial_feature,
            &c.is_working,
            &c.legal_entity as _,
            &c.file_number as _,
            &c.register_number as _,
            &c.start_date as _,
            &c.stop_date as _,
            &c.general_tax_mission as _,
            &c.value_tax_mission as _,
            &c.activity_nature as _,
            &c.activity_location as _,
            &c.record_number as _,
            &c.username as _,
            &c.password as _,
            &c.email as _,
            &id as _
        )
        .fetch_one(&mut transaction)
        .await?;

        let (from, to) = (
            Path::new("companies").join(&format!(
                "{} - {}",
                &old_company.owner, &old_company.commercial_feature
            )),
            Path::new("companies").join(format!(
                "{} - {}",
                &company.owner, &company.commercial_feature
            )),
        );

        if from.exists() {
            self.fs.write().await.rename(from, to).await?;
        }

        transaction.commit().await?;
        Ok(company)
    }

    async fn search_company(&self, s: &str) -> Result<Vec<Self::Company>, accounting_api::Error> {
        let companies = sqlx::query_as!(
            models::Company,
            r#"
                SELECT DISTINCT ON (companies.id)
                    companies.id,
                    owner,
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
                    record_number,
                    username,
                    password,
                    email
                FROM 
                    companies
                LEFT JOIN 
                    funders
                ON 
                    companies.id = funders.company_id
                WHERE 
                    companies.id::TEXT ILIKE ('%' || $1 || '%') OR
                    owner ILIKE ('%' || $1 || '%') OR
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

        Ok(companies)
    }

    async fn pay_company(&self, _c: &Self::Company, _v: f64) -> Result<Self::Company, Self::Error> {
        unimplemented!()
    }

    async fn register_user(&self, u: &RegisterUser) -> Result<Self::User, Self::Error> {
        let mut transaction = self.db.begin().await?;

        let user = sqlx::query_as!(
            models::User,
            r#"
                INSERT INTO
                    users (name, password, is_admin, value)
                VALUES
                    ($1, $2, $3, 0)
                RETURNING
                    *
            "#,
            &u.name,
            &u.password,
            &u.is_admin,
        )
        .fetch_one(&mut transaction)
        .await?;

        transaction.commit().await?;
        Ok(user)
    }
    async fn update_user(&self, id: Uuid, c: &UpdateUser) -> Result<Self::User, Self::Error> {
        let mut transaction = self.db.begin().await?;
        let user = sqlx::query_as!(
            models::User,
            r#"
                UPDATE
                    users
                SET
                    name = $2,
                    password = $3
                WHERE
                    id = $1
                RETURNING
                    *
            "#,
            &id as _,
            &c.name,
            &c.password,
        )
        .fetch_one(&mut transaction)
        .await?;

        transaction.commit().await?;
        Ok(user)
    }
    async fn get_users(&self) -> Result<Vec<Self::User>, Self::Error> {
        let users = sqlx::query_as!(
            models::User,
            r#"
                SELECT
                    *
                FROM
                    users
            "#,
        )
        .fetch_all(&self.db)
        .await?;

        Ok(users)
    }
    async fn pay_user(&self, id: Uuid, v: f64) -> Result<Self::User, Self::Error> {
        let mut transaction = self.db.begin().await?;

        let user = sqlx::query_as!(
            models::User,
            r#"
                UPDATE
                    users
                SET
                    value = $2
                WHERE
                    id = $1
                RETURNING
                    *
            "#,
            id as _,
            v,
        )
        .fetch_one(&mut transaction)
        .await?;

        transaction.commit().await?;
        Ok(user)
    }

    async fn login_user(&self, u: &LoginUser) -> Result<Self::User, Self::Error> {
        let user = sqlx::query_as!(
            models::User,
            r#"
                SELECT
                    *
                FROM
                    users
                WHERE
                    name = $1 AND password = $2
            "#,
            &u.name,
            &u.password,
        )
        .fetch_one(&self.db)
        .await?;
        Ok(user)
    }

    async fn get_user(&self, id: Uuid) -> Result<Self::User, Self::Error> {
        let user = sqlx::query_as!(
            models::User,
            r#"
                SELECT
                    *
                FROM
                    users
                WHERE
                    id = $1
            "#,
            id,
        )
        .fetch_one(&self.db)
        .await?;
        Ok(user)
    }

    async fn delete_company(&self, id: Uuid) -> Result<(), Self::Error> {
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

    async fn delete_user(&self, id: Uuid) -> Result<(), Self::Error> {
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
        user_id: Option<Uuid>,
        company_id: Option<Uuid>,
    ) -> Result<Vec<Self::Expense>, Self::Error> {
        let expenses = sqlx::query_as!(
            models::Expense,
            r#"
                SELECT
                    expenses.id,
                    expenses.value,
                    description,
                    time,
                    users.name AS "user!: _",
                    companies.commercial_feature AS "company!: _"
                FROM
                    expenses
                LEFT JOIN
                    companies
                ON
                    expenses.company_id = companies.id
                LEFT JOIN
                    users
                ON
                    expenses.user_id = users.id
                WHERE
                    (user_id = $1 OR $1 IS NULL) AND (company_id = $2 OR $2 IS NULL)
            "#,
            user_id,
            company_id,
        )
        .fetch_all(&self.db)
        .await?;

        Ok(expenses)
    }

    async fn create_expense(
        &self,
        user_id: Uuid,
        company_id: Uuid,
        expense: &CreateExpense,
    ) -> Result<Self::Expense, Self::Error> {
        if expense.value <= 0.0 {
            return Err(Self::Error::InvalidValue);
        }

        let user_value = sqlx::query!(
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
        .await?
        .value;

        if expense.value > user_value {
            return Err(Self::Error::NotEnoughUserValue(expense.value, user_value));
        }

        let mut transaction = self.db.begin().await?;

        sqlx::query!(
            r#"
                UPDATE
                    users
                SET
                    value = $2
                WHERE id = $1
            "#,
            user_id,
            user_value - expense.value,
        )
        .execute(&mut transaction)
        .await?;

        let expense = sqlx::query_as!(
            models::Expense,
            r#"
                INSERT INTO
                    expenses (user_id, company_id, value, description)
                VALUES
                    ($1, $2, $3, $4)
                RETURNING
                    id,
                    value,
                    description,
                    time,
                    (
                        SELECT
                            name
                        FROM
                            users
                        WHERE
                            id = $1
                    ) AS "user!: _",
                    (
                        SELECT
                            commercial_feature
                        FROM
                            companies
                        WHERE
                            id = $2
                    ) AS "company!: _"
            "#,
            user_id,
            company_id,
            expense.value,
            expense.description,
        )
        .fetch_one(&mut transaction)
        .await?;

        transaction.commit().await?;
        Ok(expense)
    }
    async fn delete_expense(&self, id: Uuid) -> Result<(), Self::Error> {
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
        admin_id: Option<Uuid>,
        company_id: Option<Uuid>,
    ) -> Result<Vec<Self::Income>, Self::Error> {
        let incomes = sqlx::query_as!(
            models::Income,
            r#"
                SELECT
                    incomes.id,
                    incomes.value,
                    description,
                    time,
                    users.name AS "admin!: _",
                    companies.commercial_feature AS "company!: _"
                FROM
                    incomes
                LEFT JOIN
                    companies
                ON
                    incomes.company_id = companies.id
                LEFT JOIN
                    users
                ON
                    incomes.admin_id = users.id
                WHERE
                    (admin_id = $1 OR $1 IS NULL) AND (company_id = $2 OR $2 IS NULL)
            "#,
            admin_id,
            company_id,
        )
        .fetch_all(&self.db)
        .await?;

        Ok(incomes)
    }

    async fn create_income(
        &self,
        admin_id: Uuid,
        company_id: Uuid,
        income: &CreateIncome,
    ) -> Result<Self::Income, Self::Error> {
        let mut transaction = self.db.begin().await?;

        let income = sqlx::query_as!(
            models::Income,
            r#"
                INSERT INTO
                    incomes (company_id, admin_id, value, description)
                VALUES
                    ($1, $2, $3, $4)
                RETURNING
                    id,
                    value,
                    description,
                    time,
                    (
                        SELECT
                            commercial_feature
                        FROM
                            companies
                        WHERE
                            id = $1
                    ) AS "company!: _",
                    (
                        SELECT
                            name
                        FROM
                            users
                        WHERE
                            id = $2
                    ) AS "admin!: _"
            "#,
            company_id,
            admin_id,
            income.value,
            income.description,
        )
        .fetch_one(&mut transaction)
        .await?;

        transaction.commit().await?;
        Ok(income)
    }
    async fn delete_income(&self, id: Uuid) -> Result<(), Self::Error> {
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
        company_id: Uuid,
        file: &mut TempFile<'_>,
    ) -> Result<Self::Document, Self::Error> {
        rocket::debug!("[create_document] creating {:?}", file.name_with_ext());
        let company = sqlx::query!(
            r#"
                SELECT
                    owner, commercial_feature
                FROM
                    companies
                WHERE
                    id = $1

            "#,
            company_id,
        )
        .fetch_one(&self.db)
        .await?;

        let document = models::Document::new(&company.owner, &company.commercial_feature, &file)
            .await
            .ok_or(Self::Error::Other("حدث خطأ في انشاء المستند".into()))?;

        self.fs.write().await.save(&document.path, file).await?;

        Ok(document)
    }

    async fn get_documents(&self, company_id: Uuid) -> Result<Vec<Self::Document>, Self::Error> {
        let company = sqlx::query!(
            r#"
                SELECT
                    owner, commercial_feature
                FROM
                    companies
                WHERE
                    id = $1

            "#,
            company_id,
        )
        .fetch_one(&self.db)
        .await?;

        let path = Path::new("companies")
            .join(&format!(
                "{} - {}",
                company.owner, company.commercial_feature,
            ))
            .join("documents");

        let mut documents = vec![];

        for path in self.fs.read().await.get(path).await {
            if let Some(document) =
                models::Document::new(&company.owner, &company.commercial_feature, &path.as_ref())
                    .await
            {
                documents.push(document);
            }
        }

        Ok(documents)
    }

    async fn delete_document(&self, path: impl AsRef<Path> + Send) -> Result<(), Self::Error> {
        rocket::debug!("[delete_document] deleting {:?}", path.as_ref());
        self.fs.write().await.delete(path).await?;
        Ok(())
    }

    async fn create_funder(
        &self,
        company_id: Uuid,
        f: &CreateFunder,
    ) -> Result<Self::Funder, Self::Error> {
        let mut transaction = self.db.begin().await?;

        let funder = sqlx::query_as!(
            models::Funder,
            r#"
                INSERT INTO
                    funders (
                        name, company_id
                    )
                VALUES (
                    $1, $2
                )
                RETURNING
                    id, name
            "#,
            f.name as _,
            company_id as _,
        )
        .fetch_one(&mut transaction)
        .await?;
        transaction.commit().await?;
        Ok(funder)
    }
    async fn get_funders(&self, company_id: Uuid) -> Result<Vec<Self::Funder>, Self::Error> {
        let funders = sqlx::query_as!(
            models::Funder,
            r#"
                SELECT
                    id, name
                FROM
                    funders
                WHERE
                    company_id = $1
            "#,
            company_id,
        )
        .fetch_all(&self.db)
        .await?;

        Ok(funders)
    }
    async fn delete_funder(&self, id: Uuid) -> Result<(), Self::Error> {
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
