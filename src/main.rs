#[macro_use]
extern crate rocket;

// add routes modules and helpers
mod api;
mod inc;

// load endpoints and modules
use api::status::*;
use inc::rocket::*;

#[launch]
async fn rocket() -> _ {
    // init rocket
    rocket::build()
        .register("/", catchers![internal_404, internal_500])
        .mount("/api", routes![ping])
}
