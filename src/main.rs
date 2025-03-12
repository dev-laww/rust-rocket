#[macro_use]
extern crate rocket;

mod lib;
mod models;
mod routes;

use crate::lib::builders::api_response::ApiResponseBuilder;
use crate::models::api_response::ApiResponse;
use diesel::RunQueryDsl;
use rocket::http::Status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket_sync_db_pools::{database, diesel};

#[get("/")]
async fn index() -> Json<ApiResponse<Value>> {
    let response = ApiResponseBuilder::new()
        .status(Status::Created)
        .data(json!({
            "message": "Welcome to the Rocket API",
        }))
        .build();

    Json(response)
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
