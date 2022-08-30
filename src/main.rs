#[macro_use]
extern crate rocket;

use accounting_backend::{database, routes};

#[launch]
fn rocket() -> _ {
    if cfg!(debug_assertions) {
        dotenvy::dotenv().ok();
    }

    rocket::build()
        .attach(database::stage())
        .attach(routes::stage())
}
