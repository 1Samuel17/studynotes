use sea_orm::entity::prelude::*;

// Collection entity representing a group of notebooks
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "collection")]
pub struct Model {
    #[sea_orm(unique, primary_key, auto_increment = false)]
    pub name: String,
    pub description: String,
    #[sea_orm(has_many)]
    pub notebooks: HasMany<super::notebook::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

// Unit test for the Collection entity
#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{ActiveModelTrait, Database, Set};

    // Test creating a new collection
    #[tokio::test]
    async fn test_create_collection() {
        // Set up an in-memory SQLite database for testing
        let db = Database::connect("sqlite::memory:").await.unwrap();
        db.get_schema_registry("database::models::*")
            .sync(&db)
            .await
            .unwrap();

        // Create a new collection
        let new_collection = ActiveModel {
            name: Set("Test Collection".to_string()),
            description: Set("A collection for testing".to_string()),
            ..Default::default()
        };

        // Insert the collection into the database and verify it was created correctly
        let inserted_collection = new_collection.insert(&db).await.unwrap();
        assert_eq!(inserted_collection.name, "Test Collection");
        assert_eq!(inserted_collection.description, "A collection for testing");
    }
}
