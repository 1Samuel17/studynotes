use crate::models::{collection, note, note_tag, notebook, tag};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};

/// Data describing which fields to update on an entity.
/// Only `Some` fields will be modified.
pub enum UpdateEntityData {
    Collection {
        name: Option<String>,
        description: Option<String>,
    },
    Notebook {
        name: Option<String>,
        description: Option<serde_json::Value>,
        collection_name: Option<String>,
    },
    Note {
        name: Option<String>,
        topic: Option<String>,
        content: Option<serde_json::Value>,
        tags: Option<Vec<String>>,
    },
}

/// Result of a successful update operation, typed by entity.
pub enum UpdateResult {
    Collection(collection::Model),
    Notebook(notebook::Model),
    Note(note::Model),
}

/// Update an existing record identified by `current_name`.
/// Returns `None` if the record was not found.
pub async fn update_one(
    db: &DatabaseConnection,
    current_name: &str,
    data: UpdateEntityData,
) -> Result<Option<UpdateResult>, DbErr> {
    match data {
        UpdateEntityData::Collection { name, description } => {
            let row = collection::Entity::find()
                .filter(collection::Column::Name.eq(current_name))
                .one(db)
                .await?;
            match row {
                Some(model) => {
                    let mut active: collection::ActiveModel = model.into();
                    if let Some(n) = name {
                        active.name = Set(n);
                    }
                    if let Some(d) = description {
                        active.description = Set(d);
                    }
                    let result = active.update(db).await?;
                    Ok(Some(UpdateResult::Collection(result)))
                }
                None => Ok(None),
            }
        }
        UpdateEntityData::Notebook {
            name,
            description,
            collection_name,
        } => {
            let row = notebook::Entity::find()
                .filter(notebook::Column::Name.eq(current_name))
                .one(db)
                .await?;
            match row {
                Some(model) => {
                    let mut active: notebook::ActiveModel = model.into();
                    if let Some(n) = name {
                        active.name = Set(n);
                    }
                    if let Some(d) = description {
                        active.description = Set(d);
                    }
                    if let Some(c) = collection_name {
                        active.collection_name = Set(c);
                    }
                    let result = active.update(db).await?;
                    Ok(Some(UpdateResult::Notebook(result)))
                }
                None => Ok(None),
            }
        }
        UpdateEntityData::Note {
            name,
            topic,
            content,
            tags,
        } => {
            let row = note::Entity::find()
                .filter(note::Column::Name.eq(current_name))
                .one(db)
                .await?;
            match row {
                Some(model) => {
                    let note_name = name.as_deref().unwrap_or(current_name).to_string();
                    let mut active: note::ActiveModel = model.into();
                    if let Some(n) = name {
                        active.name = Set(n);
                    }
                    if let Some(t) = topic {
                        active.topic = Set(t);
                    }
                    if let Some(c) = content {
                        active.content = Set(c);
                    }
                    let result = active.update(db).await?;

                    // Sync tags via note_tag join table
                    if let Some(tag_values) = tags {
                        // Remove existing note_tag entries for this note
                        note_tag::Entity::delete_many()
                            .filter(note_tag::Column::NoteName.eq(&note_name))
                            .exec(db)
                            .await?;
                        // Ensure each tag exists in the tag table, then insert note_tag
                        use crate::models::taxonomy::Tag as TagEnum;
                        use sea_orm::{ActiveEnum, Iterable};
                        for raw_tag in tag_values {
                            let tag_val = raw_tag.trim().to_string();
                            // Validate the tag value against known enum variants
                            let tag_enum = TagEnum::iter()
                                .find(|t| t.to_value() == tag_val);
                            let Some(tag_enum) = tag_enum else {
                                let valid: Vec<String> =
                                    TagEnum::iter().map(|t| t.to_value()).collect();
                                return Err(DbErr::Custom(format!(
                                    "Unknown tag '{}'. Valid tags: {}",
                                    tag_val,
                                    valid.join(", ")
                                )));
                            };
                            // Insert the tag if it doesn't already exist
                            let exists = tag::Entity::find()
                                .filter(tag::Column::Tag.eq(&tag_val))
                                .one(db)
                                .await?;
                            if exists.is_none() {
                                let new_tag = tag::ActiveModel {
                                    tag: Set(tag_enum),
                                    ..Default::default()
                                };
                                new_tag.insert(db).await?;
                            }
                            let nt = note_tag::ActiveModel {
                                note_name: Set(note_name.clone()),
                                tag_name: Set(tag_val),
                                ..Default::default()
                            };
                            nt.insert(db).await?;
                        }
                    }

                    Ok(Some(UpdateResult::Note(result)))
                }
                None => Ok(None),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;

    #[tokio::test]
    async fn test_update_collection() {
        let db = testutils::setup_test_db().await.unwrap();
        let data = testutils::insert_test_data(&db).await.unwrap();

        let result = update_one(
            &db,
            &data.collection.name,
            UpdateEntityData::Collection {
                name: None,
                description: Some("Updated description".to_string()),
            },
        )
        .await
        .unwrap();

        let Some(UpdateResult::Collection(col)) = result else {
            panic!("expected Collection")
        };
        assert_eq!(col.name, data.collection.name);
        assert_eq!(col.description, "Updated description");
    }

    #[tokio::test]
    async fn test_update_notebook() {
        let db = testutils::setup_test_db().await.unwrap();
        let data = testutils::insert_test_data(&db).await.unwrap();

        let result = update_one(
            &db,
            &data.notebook.name,
            UpdateEntityData::Notebook {
                name: None,
                description: Some(serde_json::json!({"text": "Updated notebook"})),
                collection_name: None,
            },
        )
        .await
        .unwrap();

        let Some(UpdateResult::Notebook(nb)) = result else {
            panic!("expected Notebook")
        };
        assert_eq!(nb.name, data.notebook.name);
        assert_eq!(
            nb.description,
            serde_json::json!({"text": "Updated notebook"})
        );
    }

    #[tokio::test]
    async fn test_update_note() {
        let db = testutils::setup_test_db().await.unwrap();
        let data = testutils::insert_test_data(&db).await.unwrap();

        let result = update_one(
            &db,
            &data.note.name,
            UpdateEntityData::Note {
                name: None,
                topic: Some("Updated Topic".to_string()),
                content: None,
                tags: None,
            },
        )
        .await
        .unwrap();

        let Some(UpdateResult::Note(n)) = result else {
            panic!("expected Note")
        };
        assert_eq!(n.name, data.note.name);
        assert_eq!(n.topic, "Updated Topic");
    }

    #[tokio::test]
    async fn test_update_note_tags() {
        use crate::models::{note_tag, taxonomy};
        use sea_orm::{ActiveEnum, EntityTrait};

        let db = testutils::setup_test_db().await.unwrap();
        let data = testutils::insert_test_data(&db).await.unwrap();

        // Update note with new tags
        let result = update_one(
            &db,
            &data.note.name,
            UpdateEntityData::Note {
                name: None,
                topic: None,
                content: None,
                tags: Some(vec![
                    taxonomy::Tag::Async.to_value(),
                    taxonomy::Tag::Testing.to_value(),
                ]),
            },
        )
        .await
        .unwrap();

        let Some(UpdateResult::Note(n)) = result else {
            panic!("expected Note")
        };
        assert_eq!(n.name, data.note.name);

        // Verify the note_tag entries were updated
        let note_tags = note_tag::Entity::find()
            .filter(note_tag::Column::NoteName.eq(&data.note.name))
            .all(&db)
            .await
            .unwrap();
        assert_eq!(note_tags.len(), 2);
        let tag_names: Vec<&str> = note_tags.iter().map(|nt| nt.tag_name.as_str()).collect();
        assert!(tag_names.contains(&taxonomy::Tag::Async.to_value().as_str()));
        assert!(tag_names.contains(&taxonomy::Tag::Testing.to_value().as_str()));
    }

    #[tokio::test]
    async fn test_update_nonexistent() {
        let db = testutils::setup_test_db().await.unwrap();

        let result = update_one(
            &db,
            "Does Not Exist",
            UpdateEntityData::Collection {
                name: None,
                description: Some("test".to_string()),
            },
        )
        .await
        .unwrap();

        assert!(result.is_none());
    }
}
