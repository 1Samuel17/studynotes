use database::connection::{set_db_options, check_db};
use sea_orm::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let db_options = set_db_options().await.unwrap();
    let db = &Database::connect(db_options).await?;
    
    check_db(db).await;

    Ok(())
}
