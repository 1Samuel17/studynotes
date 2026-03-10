use sea_orm::entity::prelude::*;
use serde_json::Value as Json;

// Note entity representing an individual note
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "note")]
pub struct Model {
    #[sea_orm(unique, primary_key, auto_increment = false)]
    pub name: String,
    pub topic: String,
    pub content: Json,
    pub notebook_name: String,
    #[sea_orm(belongs_to, from = "notebook_name", to = "name")]
    pub notebook: HasOne<super::notebook::Entity>,
    #[sea_orm(has_many, via = "note_tag")]
    pub tags: HasMany<super::tag::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

// Unit test for the Note entity
#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{ActiveModelTrait, Database, Set};

    // Test creating a new note
    #[tokio::test]
    async fn test_create_note() {
        // Set up an in-memory SQLite database for testing
        let db = Database::connect("sqlite::memory:").await.unwrap();
        db.get_schema_registry("database::models::*")
            .sync(&db)
            .await
            .unwrap();

        // Create a collection to associate with notebook
        let new_collection = super::super::collection::ActiveModel {
            name: Set("Test Collection".to_string()),
            description: Set("A collection for testing".to_string()),
            ..Default::default()
        };
        let inserted_collection = new_collection.insert(&db).await.unwrap();

        // Create a notebook to associate with the the collection and note
        let new_notebook = super::super::notebook::ActiveModel {
            name: Set("Test Notebook".to_string()),
            description: Set("A notebook for testing".to_string()),
            collection_name: Set(inserted_collection.name.clone()),
            ..Default::default()
        };
        let inserted_notebook = new_notebook.insert(&db).await.unwrap();

        // Create a new note associated with the notebook
        let new_note = ActiveModel {
            name: Set("Test Note".to_string()),
            topic: Set("Testing".to_string()),
            content: Set(serde_json::json!({"text": "This is a test note"})),
            notebook_name: Set(inserted_notebook.name.clone()),
            ..Default::default()
        };

        // Insert the note into the database and verify it was created correctly
        let inserted_note = new_note.insert(&db).await.unwrap();
        assert_eq!(inserted_note.name, "Test Note");
        assert_eq!(inserted_note.topic, "Testing");
        assert_eq!(inserted_note.content, serde_json::json!({"text": "This is a test note"}));
        assert_eq!(inserted_note.notebook_name, inserted_notebook.name);
    }
}
