#[macro_use]
extern crate rocket;

use accounting_backend::{local_storage, routes};

#[launch]
fn rocket() -> _ {
    if cfg!(debug_assertions) {
        dotenvy::dotenv().ok();
    }

    rocket::build()
        .attach(local_storage::stage())
        .attach(routes::stage())
}
