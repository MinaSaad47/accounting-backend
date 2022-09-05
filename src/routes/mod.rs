use rocket::fairing::AdHoc;

pub mod company;
pub mod funders;
pub mod documents;
pub mod expenses;
pub mod incomes;
pub mod user;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("routes stage", |rocket| async {
        rocket
            .attach(user::stage())
            .attach(company::stage())
            .attach(funders::stage())
            .attach(expenses::stage())
            .attach(incomes::stage())
            .attach(documents::stage())
    })
}
