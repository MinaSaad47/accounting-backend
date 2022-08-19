#[macro_use]
extern crate rocket;

use accounting_backend::{database, routes};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(database::stage())
        .attach(routes::stage())
}
