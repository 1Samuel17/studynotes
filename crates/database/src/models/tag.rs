use crate::models::taxonomy::Tag;
use sea_orm::entity::prelude::*;

// Tag entity representing a tag that can be associated with notes
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag: Tag,
}

impl ActiveModelBehavior for ActiveModel {}

// Unit test for the Tag model
#[cfg(test)]
mod tests {
    use crate::models::taxonomy;

    use super::*;
    use sea_orm::{ActiveModelTrait, Database, Set};

    // Test creating a new tag
    #[tokio::test]
    async fn test_create_tag() {
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
            description: Set(serde_json::json!({"text": "A notebook for testing"})),
            collection_name: Set(inserted_collection.name.clone()),
            ..Default::default()
        };
        let _inserted_notebook = new_notebook.insert(&db).await.unwrap();

        // Create a new note to associate with the tag
        let new_note = super::super::note::ActiveModel {
            name: Set("Test Note".to_string()),
            topic: Set("Testing".to_string()),
            content: Set(serde_json::json!({"text": "This is a test note"})),
            notebook_name: Set("Test Notebook".to_string()),
            ..Default::default()
        };
        let _inserted_note = new_note.insert(&db).await.unwrap();

        // Create a new tag
        let new_tag = ActiveModel {
            tag: Set(taxonomy::Tag::Important),
            ..Default::default()
        };
        let inserted_tag = new_tag.insert(&db).await.unwrap();
        assert_eq!(inserted_tag.tag, taxonomy::Tag::Important);

        // Associate the tag with the note via note_tag
        let note_tag = super::super::note_tag::ActiveModel {
            note_name: Set("Test Note".to_string()),
            tag_name: Set(taxonomy::Tag::Important.to_value()),
            ..Default::default()
        };
        note_tag.insert(&db).await.unwrap();
    }
}
