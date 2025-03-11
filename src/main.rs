#[macro_use]
extern crate rocket;

mod lib;
mod models;
mod routes;
mod schema;

use crate::lib::builders::api_response::ApiResponseBuilder;
use crate::models::api_response::ApiResponse;
use crate::models::user::{NewUser, User};
use diesel::RunQueryDsl;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_sync_db_pools::{database, diesel};

#[database("db_url")]
pub struct DbConn(diesel::PgConnection);

// TODO: Migrate to seaql orm

#[get("/")]
async fn index(conn: DbConn) -> Json<ApiResponse<User>> {
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
        .status(Status::Created)
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
