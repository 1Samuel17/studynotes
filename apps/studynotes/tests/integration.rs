// Database integration tests for models and connection setup

use database::{
    connection::{check_db, set_db_options},
    models::*,
};
use sea_orm::{ActiveModelTrait, Database, EntityTrait, Set};

// Test database connection and basic CRUD operations
#[tokio::test]
async fn test_database_integration() -> Result<(), Box<dyn std::error::Error>> {
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

    // Test database operations
    let new_collection = collection::ActiveModel {
        name: Set("My Collection".to_string()),
        description: Set("A collection of study notes".to_string()),
        ..Default::default()
    };
    let inserted_collection = new_collection.insert(db).await?;
    tracing::info!("Inserted collection: {:?}", inserted_collection);

    // Create a new notebook linked to the collection
    let new_notebook = notebook::ActiveModel {
        collection_name: Set(inserted_collection.name),
        name: Set("My Notebook".to_string()),
        description: Set("A notebook for my study notes".to_string()),
        ..Default::default()
    };
    let inserted_notebook = new_notebook.insert(db).await?;
    tracing::info!("Inserted notebook: {:?}", inserted_notebook);

    // Create a new note linked to the notebook
    let new_note = note::ActiveModel {
        notebook_name: Set(inserted_notebook.name.clone()),
        name: Set("My First Note".to_string()),
        topic: Set("General".to_string()),
        content: Set("This is the content of my first note.".to_string()),
        ..Default::default()
    };
    let inserted_note = new_note.insert(db).await?;
    tracing::info!("Inserted note: {:?}", inserted_note);

    // Create a new tag
    let new_tag = tag::ActiveModel {
        tag: Set(taxonomy::Tag::Important),
        note_name: Set(inserted_note.name.clone()),
        ..Default::default()
    };
    let inserted_tag = new_tag.insert(db).await?;
    tracing::info!("Inserted tag: {:?}", inserted_tag);

    // Delete inserted data to clean up after the test (cascading deletes should handle related records first)
    tag::Entity::delete_many().exec(db).await?;
    note::Entity::delete_many().exec(db).await?;
    notebook::Entity::delete_many().exec(db).await?;
    collection::Entity::delete_many().exec(db).await?;

    Ok(())
}
