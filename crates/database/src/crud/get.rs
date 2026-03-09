use crate::models::{collection, note, notebook, tag};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter, ColumnTrait};

/// Describes which entity to retrieve from the database.
pub enum EntityKind {
    Collection,
    Notebook,
    Note,
    Tag,
}

/// Holds the result of a `get_all` query, typed by entity.
#[derive(Debug)]
pub enum GetAllQueryResult {
    Collections(Vec<collection::Model>),
    Notebooks(Vec<notebook::Model>),
    Notes(Vec<note::Model>),
    Tags(Vec<tag::Model>),
}

/// Holds the result of a `get_by_name` query, typed by entity.
#[derive(Debug)]
pub enum GetByNameQueryResult {
    Collection(collection::Model),
    Notebook(notebook::Model),
    Note(note::Model),
    Tag(tag::Model),
}

/// Retrieve all records for the given entity kind.
pub async fn get_all(db: &DatabaseConnection, kind: EntityKind) -> Result<GetAllQueryResult, DbErr> {
    match kind {
        EntityKind::Collection => {
            let rows = collection::Entity::find().all(db).await?;
            Ok(GetAllQueryResult::Collections(rows))
        }
        EntityKind::Notebook => {
            let rows = notebook::Entity::find().all(db).await?;
            Ok(GetAllQueryResult::Notebooks(rows))
        }
        EntityKind::Note => {
            let rows = note::Entity::find().all(db).await?;
            Ok(GetAllQueryResult::Notes(rows))
        }
        EntityKind::Tag => {
            let rows = tag::Entity::find().all(db).await?;
            Ok(GetAllQueryResult::Tags(rows))
        }
    }
}

/// Retrieve a single record by its entity kind and name.
pub async fn get_by_name(db: &DatabaseConnection, kind: EntityKind, name: &str) -> Result<Option<GetByNameQueryResult>, DbErr> {
    match kind {
        EntityKind::Collection => {
            let row = collection::Entity::find().filter(collection::Column::Name.eq(name)).one(db).await?;
            Ok(row.map(GetByNameQueryResult::Collection))
        }
        EntityKind::Notebook => {
            let row = notebook::Entity::find().filter(notebook::Column::Name.eq(name)).one(db).await?;
            Ok(row.map(GetByNameQueryResult::Notebook))
        }
        EntityKind::Note => {
            let row = note::Entity::find().filter(note::Column::Name.eq(name)).one(db).await?;
            Ok(row.map(GetByNameQueryResult::Note))
        }
        EntityKind::Tag => {
            let row = tag::Entity::find().filter(tag::Column::Tag.eq(name)).one(db).await?;
            Ok(row.map(GetByNameQueryResult::Tag))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use sea_orm::ActiveEnum;

    #[tokio::test]
    async fn test_get_all() {
        let db = testutils::setup_test_db().await.unwrap();
        let data = testutils::insert_test_data(&db).await.unwrap();

        // Collections
        let result = get_all(&db, EntityKind::Collection).await.unwrap();
        let GetAllQueryResult::Collections(collections) = result else { panic!("expected Collections") };
        assert!(!collections.is_empty());
        assert!(collections.iter().any(|c| c.name == data.collection.name));

        // Notebooks
        let result = get_all(&db, EntityKind::Notebook).await.unwrap();
        let GetAllQueryResult::Notebooks(notebooks) = result else { panic!("expected Notebooks") };
        assert!(!notebooks.is_empty());
        assert!(notebooks.iter().any(|n| n.name == data.notebook.name));

        // Notes
        let result = get_all(&db, EntityKind::Note).await.unwrap();
        let GetAllQueryResult::Notes(notes) = result else { panic!("expected Notes") };
        assert!(!notes.is_empty());
        assert!(notes.iter().any(|n| n.name == data.note.name));

        // Tags
        let result = get_all(&db, EntityKind::Tag).await.unwrap();
        let GetAllQueryResult::Tags(tags) = result else { panic!("expected Tags") };
        assert!(!tags.is_empty());
        assert!(tags.iter().any(|t| t.note_name == data.tag.note_name && t.tag == data.tag.tag));

        testutils::clear_test_data(&db).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_by_name() {
        let db = testutils::setup_test_db().await.unwrap();
        let data = testutils::insert_test_data(&db).await.unwrap();

        // Collection
        let result = get_by_name(&db, EntityKind::Collection, &data.collection.name).await.unwrap();
        let Some(GetByNameQueryResult::Collection(collection)) = result else { panic!("expected Collection") };
        assert_eq!(collection.name, data.collection.name);

        // Notebook
        let result = get_by_name(&db, EntityKind::Notebook, &data.notebook.name).await.unwrap();
        let Some(GetByNameQueryResult::Notebook(notebook)) = result else { panic!("expected Notebook") };
        assert_eq!(notebook.name, data.notebook.name);

        // Note
        let result = get_by_name(&db, EntityKind::Note, &data.note.name).await.unwrap();
        let Some(GetByNameQueryResult::Note(note)) = result else { panic!("expected Note") };
        assert_eq!(note.name, data.note.name);

        // Tag
        let result = get_by_name(&db, EntityKind::Tag, &data.tag.tag.to_value()).await.unwrap();
        let Some(GetByNameQueryResult::Tag(tag)) = result else { panic!("expected Tag") };
        assert_eq!(tag.tag, data.tag.tag);
        assert_eq!(tag.note_name, data.tag.note_name);

        testutils::clear_test_data(&db).await.unwrap();
    }
}