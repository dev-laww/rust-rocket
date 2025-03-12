#[macro_use]
extern crate rocket;

mod lib;
mod models;
mod routes;
mod setup;

use crate::lib::builders::api_response::ApiResponseBuilder;
use crate::models::api_response::ApiResponse;
use models::{prelude::*, *};
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

#[get("/")]
async fn index(db: &State<DatabaseConnection>) -> Json<ApiResponse<Value>> {
    let db = db as &DatabaseConnection;

    let user = user::ActiveModel {
        first_name: ActiveValue::Set("John".to_owned()),
        middle_name: ActiveValue::Set("Doe".to_owned()),
        last_name: ActiveValue::Set("Smith".to_owned()),
        email: ActiveValue::Set("@example.com".to_owned()),
        password: ActiveValue::Set("password".to_owned()),
        ..Default::default()
    };

    let insert_result = User::insert(user).exec(db).await;

    let inserted_user = match User::find_by_id(insert_result.unwrap().last_insert_id).one(db).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            let response = ApiResponseBuilder::new()
                .status(Status::NotFound)
                .message("User not found")
                .build();
            return Json(response);
        },
        Err(err) => {
            let response = ApiResponseBuilder::new()
                .status(Status::InternalServerError)
                .message(&*err.to_string())
                .build();
            return Json(response);
        }
    };

    let response = ApiResponseBuilder::new()
        .status(Status::Created)
        .data(json!(inserted_user))
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
