use sea_orm::entity::prelude::*;

// Notebook entity representing a collection of notes
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "notebook")]
pub struct Model {
    #[sea_orm(unique, primary_key, auto_increment = false)]
    pub name: String,
    pub description: String,
    pub collection_name: String,
    #[sea_orm(belongs_to, from = "collection_name", to = "name")]
    pub collection: HasOne<super::collection::Entity>,
    #[sea_orm(has_many)]
    pub notes: HasMany<super::note::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

// Unit test for the Notebook entity
#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{ActiveModelTrait, Database, Set};

    // Test creating a new notebook
    #[tokio::test]
    async fn test_create_notebook() {
        // Set up an in-memory SQLite database for testing
        let db = Database::connect("sqlite::memory:").await.unwrap();
        db.get_schema_registry("database::models::*")
            .sync(&db)
            .await
            .unwrap();

        // Create a collection to associate with the notebook
        let new_collection = super::super::collection::ActiveModel {
            name: Set("Test Collection".to_string()),
            description: Set("A collection for testing".to_string()),
            ..Default::default()
        };
        let inserted_collection = new_collection.insert(&db).await.unwrap();

        // Create a new notebook associated with the collection
        let new_notebook = ActiveModel {
            name: Set("Test Notebook".to_string()),
            description: Set("A notebook for testing".to_string()),
            collection_name: Set(inserted_collection.name),
            ..Default::default()
        };

        // Insert the notebook into the database and verify it was created correctly
        let inserted_notebook = new_notebook.insert(&db).await.unwrap();
        assert_eq!(inserted_notebook.name, "Test Notebook");
        assert_eq!(inserted_notebook.description, "A notebook for testing");
        assert_eq!(inserted_notebook.collection_name, "Test Collection");
    }
}
