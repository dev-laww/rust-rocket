#[macro_use]
extern crate rocket;

mod lib;
mod models;
mod routes;

use rocket::serde::json::{json, Json, Value};

#[get("/")]
fn index() -> Json<Value>{
    let json =json!({
        "status": 200,
        "message": "Welcome to the Rocket API",
    });

    Json(json)
}

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().expect("Failed to read .env file");

    let config = rocket::Config::figment().merge((
        "databases.db_url.url",
        std::env::var("DATABASE_URL").unwrap(),
    ));

    rocket::custom(config)
        .attach(DbConn::fairing())
        .mount("/", routes![index])
}
