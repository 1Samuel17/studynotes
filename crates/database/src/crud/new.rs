use crate::models::{collection, note, note_tag, notebook, tag, taxonomy};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};

/// Data required to create a new entity.
pub enum NewEntityData {
    Collection {
        name: String,
        description: String,
    },
    Notebook {
        name: String,
        description: serde_json::Value,
        collection_name: String,
    },
    Note {
        name: String,
        topic: String,
        content: serde_json::Value,
        notebook_name: String,
    },
    Tag {
        tag: taxonomy::Tag,
    },
    NoteTag {
        note_name: String,
        tag_name: String,
    },
}

/// Result of a successful create operation, typed by entity.
pub enum CreateResult {
    Collection(collection::Model),
    Notebook(notebook::Model),
    Note(note::Model),
    Tag(tag::Model),
    NoteTag(note_tag::Model),
}

/// Insert a new record into the database.
pub async fn create_one(
    db: &DatabaseConnection,
    data: NewEntityData,
) -> Result<CreateResult, DbErr> {
    match data {
        NewEntityData::Collection { name, description } => {
            let model = collection::ActiveModel {
                name: Set(name),
                description: Set(description),
                ..Default::default()
            };
            let result = model.insert(db).await?;
            Ok(CreateResult::Collection(result))
        }
        NewEntityData::Notebook {
            name,
            description,
            collection_name,
        } => {
            let model = notebook::ActiveModel {
                name: Set(name),
                description: Set(description),
                collection_name: Set(collection_name),
                ..Default::default()
            };
            let result = model.insert(db).await?;
            Ok(CreateResult::Notebook(result))
        }
        NewEntityData::Note {
            name,
            topic,
            content,
            notebook_name,
        } => {
            let model = note::ActiveModel {
                name: Set(name),
                topic: Set(topic),
                content: Set(content),
                notebook_name: Set(notebook_name),
                ..Default::default()
            };
            let result = model.insert(db).await?;
            Ok(CreateResult::Note(result))
        }
        NewEntityData::Tag { tag: tag_val } => {
            let model = tag::ActiveModel {
                tag: Set(tag_val),
                ..Default::default()
            };
            let result = model.insert(db).await?;
            Ok(CreateResult::Tag(result))
        }
        NewEntityData::NoteTag {
            note_name,
            tag_name,
        } => {
            let model = note_tag::ActiveModel {
                note_name: Set(note_name),
                tag_name: Set(tag_name),
                ..Default::default()
            };
            let result = model.insert(db).await?;
            Ok(CreateResult::NoteTag(result))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils;
    use sea_orm::ActiveEnum;

    #[tokio::test]
    async fn test_create_collection() {
        let db = testutils::setup_test_db().await.unwrap();

        let result = create_one(
            &db,
            NewEntityData::Collection {
                name: "New Collection".to_string(),
                description: "A brand new collection".to_string(),
            },
        )
        .await
        .unwrap();

        let CreateResult::Collection(col) = result else {
            panic!("expected Collection")
        };
        assert_eq!(col.name, "New Collection");
        assert_eq!(col.description, "A brand new collection");
    }

    #[tokio::test]
    async fn test_create_notebook() {
        let db = testutils::setup_test_db().await.unwrap();
        let data = testutils::insert_test_data(&db).await.unwrap();

        let result = create_one(
            &db,
            NewEntityData::Notebook {
                name: "New Notebook".to_string(),
                description: serde_json::json!({"text": "A new notebook"}),
                collection_name: data.collection.name.clone(),
            },
        )
        .await
        .unwrap();

        let CreateResult::Notebook(nb) = result else {
            panic!("expected Notebook")
        };
        assert_eq!(nb.name, "New Notebook");
        assert_eq!(nb.collection_name, data.collection.name);
    }

    #[tokio::test]
    async fn test_create_note() {
        let db = testutils::setup_test_db().await.unwrap();
        let data = testutils::insert_test_data(&db).await.unwrap();

        let result = create_one(
            &db,
            NewEntityData::Note {
                name: "New Note".to_string(),
                topic: "Testing".to_string(),
                content: serde_json::json!({"text": "New note content"}),
                notebook_name: data.notebook.name.clone(),
            },
        )
        .await
        .unwrap();

        let CreateResult::Note(n) = result else {
            panic!("expected Note")
        };
        assert_eq!(n.name, "New Note");
        assert_eq!(n.topic, "Testing");
        assert_eq!(n.notebook_name, data.notebook.name);
    }

    #[tokio::test]
    async fn test_create_tag() {
        let db = testutils::setup_test_db().await.unwrap();

        let result = create_one(
            &db,
            NewEntityData::Tag {
                tag: taxonomy::Tag::Async,
            },
        )
        .await
        .unwrap();

        let CreateResult::Tag(t) = result else {
            panic!("expected Tag")
        };
        assert_eq!(t.tag, taxonomy::Tag::Async);
    }

    #[tokio::test]
    async fn test_create_note_tag() {
        let db = testutils::setup_test_db().await.unwrap();
        let data = testutils::insert_test_data(&db).await.unwrap();

        // First insert the tag
        create_one(
            &db,
            NewEntityData::Tag {
                tag: taxonomy::Tag::Async,
            },
        )
        .await
        .unwrap();

        let result = create_one(
            &db,
            NewEntityData::NoteTag {
                note_name: data.note.name.clone(),
                tag_name: taxonomy::Tag::Async.to_value(),
            },
        )
        .await
        .unwrap();

        let CreateResult::NoteTag(nt) = result else {
            panic!("expected NoteTag")
        };
        assert_eq!(nt.note_name, data.note.name);
    }
}
