#[macro_use]
extern crate rocket;

use accounting_backend::{db, routes};

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(db::stage())
        .attach(routes::company::stage())
        .attach(routes::user::stage())
}
