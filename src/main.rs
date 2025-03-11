#[macro_use]
extern crate rocket;

mod lib;
mod routes;
mod schema;
mod models;

use diesel::RunQueryDsl;
use http::StatusCode;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket_sync_db_pools::{database, diesel};
use crate::lib::builders::api_response::ApiResponseBuilder;
use crate::models::api_response::ApiResponse;
use crate::models::user::{NewUser, User};

#[database("db_url")]
pub struct DbConn(diesel::PgConnection);

#[get("/")]
async fn index(conn: DbConn) -> Json<ApiResponse<User>>{
    let json =json!({
        "status": 200,
        "message": "Welcome to the Rocket API",
    });

    let user = NewUser {
        username: "admin".to_string(),
        email: "admin@localhost".to_string(),
        password: "admin".to_string(),
    };

    let inserted_user = conn
        .run(move |c| {
            diesel::insert_into(schema::users::table)
                .values(&user)
                .get_result::<User>(c)
        })
        .await
        .expect("Error saving new user");

    let response = ApiResponseBuilder::new()
        .status(StatusCode::CREATED)
        .data(inserted_user)
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
