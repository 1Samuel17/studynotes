// use clap::{Parser, Subcommand, Args};
use database::connection::{check_db, set_db_options};
use sea_orm::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging with tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    // Set up the database connection
    let db_options = set_db_options().await.unwrap();
    let db = &Database::connect(db_options).await?;

    // Check the database connection
    check_db(db).await;

    // synchronizes database schema with entity definitions
    db.get_schema_registry("database::models::*")
        .sync(db)
        .await?;

    Ok(())
}
