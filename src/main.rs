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
    rocket::build().mount("/", routes![index])
}
