use sea_orm::*;

pub(super) async fn setup() -> Result<DatabaseConnection, DbErr> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url).await?;
    Ok(db)
}