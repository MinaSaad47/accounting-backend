use rocket::fairing::AdHoc;

pub mod company;
pub mod user;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("routes stage", |rocket| async {
        rocket.attach(user::stage()).attach(company::stage())
    })
}
