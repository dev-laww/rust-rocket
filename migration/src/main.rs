use sea_orm_migration::prelude::*;
use sqlx::migrate::MigrateDatabase;
use sqlx::Postgres;

#[async_std::main]
async fn main() {
    // check if database exists
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");

    if !Postgres::database_exists(&db_url).await.unwrap() {
        println!(
            "Database does not exist. Please create the database first. Creating the database..."
        );
        Postgres::create_database(&db_url).await.unwrap();
    }

    cli::run_cli(migration::Migrator).await;
}
