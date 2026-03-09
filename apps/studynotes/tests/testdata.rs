use anyhow::Result;
use database::{connection::{check_db, set_db_options}, testutils};
use sea_orm::{Database, DatabaseConnection};

pub use testutils::TestData;

/// Set up the real database for integration testing.
/// Uses DATABASE_URL from .env and initializes tracing.
pub async fn setup_test_db() -> Result<DatabaseConnection> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let db_options = set_db_options().await.unwrap();
    let db = Database::connect(db_options).await?;
    check_db(&db).await;

    db.get_schema_registry("database::models::*")
        .sync(&db)
        .await?;

    Ok(db)
}

/// Insert standard test data, returning the inserted models.
pub async fn insert_test_data(db: &DatabaseConnection) -> Result<TestData> {
    testutils::insert_test_data(db).await
}

/// Clear all test data from the database.
pub async fn clear_test_data(db: &DatabaseConnection) -> Result<()> {
    testutils::clear_test_data(db).await
}