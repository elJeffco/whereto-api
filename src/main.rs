#[macro_use]
extern crate rocket;
use dotenv::dotenv;

// add routes modules and helpers
mod api;
mod inc;

// load endpoints and modules
use api::status::*;
use inc::rocket::*;

#[launch]
async fn rocket() -> _ {
    // init dotenv variables
    dotenv().ok();

    // init rocket
    rocket::build()
        .register("/", catchers![internal_404, internal_500])
        .mount("/api", routes![ping])
}
