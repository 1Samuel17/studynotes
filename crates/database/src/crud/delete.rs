use crate::crud::EntityKind;
use crate::models::{collection, note, notebook, tag};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter};

/// Delete a single record by entity kind and name.
/// Returns `true` if a record was deleted, `false` if not found.
pub async fn delete_one(
    db: &DatabaseConnection,
    kind: EntityKind,
    name: &str,
) -> Result<bool, DbErr> {
    match kind {
        EntityKind::Collection => {
            let row = collection::Entity::find()
                .filter(collection::Column::Name.eq(name))
                .one(db)
                .await?;
            match row {
                Some(model) => {
                    model.delete(db).await?;
                    Ok(true)
                }
                None => Ok(false),
            }
        }
        EntityKind::Notebook => {
            let row = notebook::Entity::find()
                .filter(notebook::Column::Name.eq(name))
                .one(db)
                .await?;
            match row {
                Some(model) => {
                    model.delete(db).await?;
                    Ok(true)
                }
                None => Ok(false),
            }
        }
        EntityKind::Note => {
            let row = note::Entity::find()
                .filter(note::Column::Name.eq(name))
                .one(db)
                .await?;
            match row {
                Some(model) => {
                    model.delete(db).await?;
                    Ok(true)
                }
                None => Ok(false),
            }
        }
        EntityKind::Tag => {
            let row = tag::Entity::find()
                .filter(tag::Column::Tag.eq(name))
                .one(db)
                .await?;
            match row {
                Some(model) => {
                    model.delete(db).await?;
                    Ok(true)
                }
                None => Ok(false),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::note_tag;
    use crate::testutils;
    use sea_orm::EntityTrait;

    #[tokio::test]
    async fn test_delete_collection() {
        let db = testutils::setup_test_db().await.unwrap();
        let data = testutils::insert_test_data(&db).await.unwrap();

        // Delete dependents in FK order: note_tags -> notes -> notebooks -> collection
        note_tag::Entity::delete_many().exec(&db).await.unwrap();
        delete_one(&db, EntityKind::Note, &data.note.name)
            .await
            .unwrap();
        delete_one(&db, EntityKind::Notebook, &data.notebook.name)
            .await
            .unwrap();

        let deleted = delete_one(&db, EntityKind::Collection, &data.collection.name)
            .await
            .unwrap();
        assert!(deleted);

        let rows = collection::Entity::find().all(&db).await.unwrap();
        assert!(rows.is_empty());
    }

    #[tokio::test]
    async fn test_delete_note() {
        let db = testutils::setup_test_db().await.unwrap();
        let data = testutils::insert_test_data(&db).await.unwrap();

        // Delete note_tag associations first due to FK constraint
        note_tag::Entity::delete_many().exec(&db).await.unwrap();

        let deleted = delete_one(&db, EntityKind::Note, &data.note.name)
            .await
            .unwrap();
        assert!(deleted);

        let rows = note::Entity::find().all(&db).await.unwrap();
        assert!(rows.is_empty());
    }

    #[tokio::test]
    async fn test_delete_nonexistent() {
        let db = testutils::setup_test_db().await.unwrap();

        let deleted = delete_one(&db, EntityKind::Collection, "Does Not Exist")
            .await
            .unwrap();
        assert!(!deleted);
    }
}
