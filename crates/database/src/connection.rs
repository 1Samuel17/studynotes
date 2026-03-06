// Database connection handled by SeaORM

use anyhow::{Context, Result};
use sea_orm::{ConnectOptions, DatabaseConnection};
use std::time::Duration;

// Set up database connection options
pub async fn set_db_options() -> Result<ConnectOptions> {
    let mut opt = ConnectOptions::new(
        dotenvy::var("DATABASE_URL").context("DATABASE_URL must be set in .env file")?,
    );
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false) // disable SQLx logging
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("studynotes"); // set default Postgres schema

    Ok(opt)
}

// Check database connection by pinging it
pub async fn check_db(db: &DatabaseConnection) {
    assert!(db.ping().await.is_ok());
    // let _ = db.clone().close().await;
    // assert!(matches!(db.ping().await, Err(DbErr::ConnectionAcquire(_))));
    println!("Database connection Ok");
}
