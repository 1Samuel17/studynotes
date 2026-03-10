// Database integration tests for models and connection setup

mod testdata;

// Test database connection and basic CRUD operations
#[tokio::test]
async fn test_database_integration() -> Result<(), Box<dyn std::error::Error>> {
    let db = testdata::setup_test_db().await?;

    // Clear any leftover data from previous runs
    testdata::clear_test_data(&db).await?;

    let data = testdata::insert_test_data(&db).await?;
    tracing::info!("Inserted collection: {:?}", data.collection);
    tracing::info!("Inserted notebook: {:?}", data.notebook);
    tracing::info!("Inserted note: {:?}", data.note);
    tracing::info!("Inserted tag: {:?}", data.tag);

    testdata::clear_test_data(&db).await?;

    Ok(())
}
