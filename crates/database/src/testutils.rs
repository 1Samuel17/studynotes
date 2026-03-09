use crate::models::*;
use anyhow::Result;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait, Set};

/// Holds the inserted test data models for use in assertions.
pub struct TestData {
    pub collection: collection::Model,
    pub notebook: notebook::Model,
    pub note: note::Model,
    pub tag: tag::Model,
}

/// Set up an in-memory SQLite database with schema synced for testing.
pub async fn setup_test_db() -> Result<DatabaseConnection> {
    let db = Database::connect("sqlite::memory:").await?;
    db.get_schema_registry("database::models::*")
        .sync(&db)
        .await?;
    Ok(db)
}

/// Insert a standard set of test data and return the inserted models.
pub async fn insert_test_data(db: &DatabaseConnection) -> Result<TestData> {
    let new_collection = collection::ActiveModel {
        name: Set("My Collection".to_string()),
        description: Set("A collection of study notes".to_string()),
        ..Default::default()
    };
    let inserted_collection = new_collection.insert(db).await?;

    let new_notebook = notebook::ActiveModel {
        collection_name: Set(inserted_collection.name.clone()),
        name: Set("My Notebook".to_string()),
        description: Set("A notebook for my study notes".to_string()),
        ..Default::default()
    };
    let inserted_notebook = new_notebook.insert(db).await?;

    let new_note = note::ActiveModel {
        notebook_name: Set(inserted_notebook.name.clone()),
        name: Set("My First Note".to_string()),
        topic: Set("General".to_string()),
        content: Set("This is the content of my first note.".to_string()),
        ..Default::default()
    };
    let inserted_note = new_note.insert(db).await?;

    let new_tag = tag::ActiveModel {
        tag: Set(taxonomy::Tag::Important),
        note_name: Set(inserted_note.name.clone()),
        ..Default::default()
    };
    let inserted_tag = new_tag.insert(db).await?;

    Ok(TestData {
        collection: inserted_collection,
        notebook: inserted_notebook,
        note: inserted_note,
        tag: inserted_tag,
    })
}

/// Clear all test data from the database.
pub async fn clear_test_data(db: &DatabaseConnection) -> Result<()> {
    tag::Entity::delete_many().exec(db).await?;
    note::Entity::delete_many().exec(db).await?;
    notebook::Entity::delete_many().exec(db).await?;
    collection::Entity::delete_many().exec(db).await?;
    Ok(())
}
