#[macro_use]
extern crate rocket;

mod lib;
mod models;
mod routes;
mod setup;

use crate::lib::builders::api_response::ApiResponseBuilder;
use crate::models::api_response::ApiResponse;
use diesel::RunQueryDsl;
use models::{prelude::*, *};
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket_sync_db_pools::{database, diesel};
use sea_orm::{DatabaseConnection, EntityTrait};

#[get("/")]
async fn index(db: &State<DatabaseConnection>) -> Json<ApiResponse<Value>> {
    let db = db as &DatabaseConnection;

    let bakery_names = User::find()
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|user| user.first_name)
        .collect::<Vec<_>>();

    let response = ApiResponseBuilder::new()
        .status(Status::Created)
        .data(json!(bakery_names))
        .build();

    Json(response)
}

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().expect("Failed to read .env file");

    let db = match setup::setup().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build().manage(db).mount("/", routes![index])
}
