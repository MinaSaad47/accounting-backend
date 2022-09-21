use accounting_backend::{
    database::{models, rows, LocalStorageAccountingApi, DB},
    routes,
    types::response::Content,
};

use rocket::{
    http::{ContentType, Status},
    local::asynchronous::Client,
};

use std::error::Error;

use sqlx::pool::Pool;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("db/migrations");

#[sqlx::test(migrator = "MIGRATOR")]
async fn create_company_test(pool: Pool<DB>) -> Result<(), Box<dyn Error>> {
    let client = Client::tracked(
        rocket::build()
            .manage(LocalStorageAccountingApi { db: pool })
            .attach(routes::stage()),
    )
    .await?;

    let company_request = models::Company {
        company: rows::Company {
            email: "email".into(),
            is_working: false,
            natural_id: Some("2".into()),
            ..Default::default()
        },
        funders: vec![rows::Funder::default()],
        ..Default::default()
    };

    let request = client
        .post("/api/company")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&company_request)?);

    let response = request.dispatch().await;

    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let company_response = response
        .into_json::<Content<models::Company>>()
        .await
        .ok_or_else(|| "could not deserialize response")?;

    assert!(company_response.status);
    Ok(())
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn search_company_test(pool: Pool<DB>) -> Result<(), Box<dyn Error>> {
    let client = Client::tracked(
        rocket::build()
            .manage(LocalStorageAccountingApi { db: pool })
            .attach(routes::stage()),
    )
    .await?;

    let create_bodies = [
        models::Company {
            company: rows::Company {
                email: "email".into(),
                is_working: false,
                natural_id: Some("2".into()),
                ..Default::default()
            },
            funders: vec![rows::Funder {
                name: "Mina".into(),
                ..Default::default()
            }],
            ..Default::default()
        },
        models::Company {
            company: rows::Company {
                email: "email".into(),
                is_working: false,
                natural_id: Some("2".into()),
                ..Default::default()
            },
            funders: vec![rows::Funder {
                name: "Fadi".into(),
                ..Default::default()
            }],
            ..Default::default()
        },
        models::Company {
            company: rows::Company {
                email: "email".into(),
                is_working: false,
                natural_id: Some("2".into()),
                ..Default::default()
            },
            funders: vec![rows::Funder {
                name: "Shady".into(),
                ..Default::default()
            }],
            ..Default::default()
        },
    ];

    for body in &create_bodies {
        let request = client
            .post("/api/company")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&body)?);
        let response = request.dispatch().await;
        assert_eq!(response.status(), Status::Created);
    }

    let request = client.get("/api/company?search=ad");

    let response = request.dispatch().await;

    // assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let content: Content<Vec<models::Company>> = response
        .into_json()
        .await
        .ok_or_else(|| "could not deserialize response")?;

    assert_eq!(
        content
            .data
            .into_iter()
            .flatten()
            .collect::<Vec<models::Company>>()
            .len(),
        2
    );

    Ok(())
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn update_company_test(pool: Pool<DB>) -> Result<(), Box<dyn Error>> {
    let client = Client::tracked(
        rocket::build()
            .manage(LocalStorageAccountingApi { db: pool })
            .attach(routes::stage()),
    )
    .await?;

    let company_request = models::Company {
        company: rows::Company {
            email: "email".into(),
            is_working: false,
            natural_id: Some("2".into()),
            ..Default::default()
        },
        funders: vec![rows::Funder::default()],
        ..Default::default()
    };

    let request = client
        .post("/api/company")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&company_request)?);

    let response = request.dispatch().await;

    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let company_response = response
        .into_json::<Content<models::Company>>()
        .await
        .ok_or_else(|| "could not deserialize response")?;

    let mut company = company_response.data.expect("response company");
    company.company.is_working = true;
    company.company.user_name = "new email".into();

    let request = client
        .put("/api/company/")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&company)?);

    let response = request.dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let company_response = response
        .into_json::<Content<models::Company>>()
        .await
        .ok_or_else(|| "could not deserialize response")?;

    let update_company = company_response.data.expect("response company");

    assert!(company_response.status);
    assert_eq!(update_company.company.email, company.company.email);
    assert_eq!(
        update_company.company.is_working,
        company.company.is_working
    );
    Ok(())
}
