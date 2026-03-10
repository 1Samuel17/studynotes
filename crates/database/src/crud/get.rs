use crate::models::{collection, note, note_tag, notebook, tag, taxonomy};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, QueryFilter};

/// Describes which entity to retrieve from the database.
pub enum EntityKind {
    Collection,
    Notebook,
    Note,
    Tag,
}

/// Summary view of a note (for listing).
#[derive(Debug)]
pub struct NoteSummary {
    pub name: String,
    pub topic: String,
    pub notebook_name: String,
}

/// Detailed view of a note (for show).
#[derive(Debug)]
pub struct NoteDetail {
    pub name: String,
    pub topic: String,
    pub content: String,
    pub notebook_name: String,
    pub collection_name: String,
    pub tags: Vec<taxonomy::Tag>,
}

/// Detailed view of a notebook (for show), including its notes.
#[derive(Debug)]
pub struct NotebookDetail {
    pub name: String,
    pub description: String,
    pub collection_name: String,
    pub notes: Vec<NoteSummary>,
}

/// Holds the result of a `get_all` query, typed by entity.
#[derive(Debug)]
pub enum GetAllQueryResult {
    Collections(Vec<collection::Model>),
    Notebooks(Vec<notebook::Model>),
    Notes(Vec<NoteSummary>),
    Tags(Vec<tag::Model>),
}

/// Holds the result of a `get_by_name` query, typed by entity.
#[derive(Debug)]
pub enum GetByNameQueryResult {
    Collection(collection::Model),
    Notebook(NotebookDetail),
    Note(NoteDetail),
    Tag(tag::Model),
}

/// Retrieve all records for the given entity kind.
pub async fn get_all(
    db: &DatabaseConnection,
    kind: EntityKind,
) -> Result<GetAllQueryResult, DbErr> {
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
            let summaries = rows
                .into_iter()
                .map(|n| NoteSummary {
                    name: n.name,
                    topic: n.topic,
                    notebook_name: n.notebook_name,
                })
                .collect();
            Ok(GetAllQueryResult::Notes(summaries))
        }
        EntityKind::Tag => {
            let rows = tag::Entity::find().all(db).await?;
            Ok(GetAllQueryResult::Tags(rows))
        }
    }
}

/// Retrieve a single record by its entity kind and name.
pub async fn get_by_name(
    db: &DatabaseConnection,
    kind: EntityKind,
    name: &str,
) -> Result<Option<GetByNameQueryResult>, DbErr> {
    match kind {
        EntityKind::Collection => {
            let row = collection::Entity::find()
                .filter(collection::Column::Name.eq(name))
                .one(db)
                .await?;
            Ok(row.map(GetByNameQueryResult::Collection))
        }
        EntityKind::Notebook => {
            let row = notebook::Entity::find()
                .filter(notebook::Column::Name.eq(name))
                .one(db)
                .await?;
            match row {
                Some(nb) => {
                    let notes = nb.find_related(note::Entity).all(db).await?;
                    let note_summaries = notes
                        .into_iter()
                        .map(|n| NoteSummary {
                            name: n.name,
                            topic: n.topic,
                            notebook_name: n.notebook_name,
                        })
                        .collect();
                    Ok(Some(GetByNameQueryResult::Notebook(NotebookDetail {
                        name: nb.name,
                        description: nb.description,
                        collection_name: nb.collection_name,
                        notes: note_summaries,
                    })))
                }
                None => Ok(None),
            }
        }
        EntityKind::Note => {
            let row = note::Entity::find()
                .filter(note::Column::Name.eq(name))
                .one(db)
                .await?;
            match row {
                Some(n) => {
                    // Get tags via note_tag join table
                    let tag_rows = note_tag::Entity::find()
                        .filter(note_tag::Column::NoteName.eq(&n.name))
                        .all(db)
                        .await?;
                    let mut tags = Vec::new();
                    for nt in tag_rows {
                        if let Some(t) = tag::Entity::find()
                            .filter(tag::Column::Tag.eq(&nt.tag_name))
                            .one(db)
                            .await?
                        {
                            tags.push(t.tag);
                        }
                    }
                    // Get collection name via notebook
                    let collection_name = notebook::Entity::find()
                        .filter(notebook::Column::Name.eq(&n.notebook_name))
                        .one(db)
                        .await?
                        .map(|nb| nb.collection_name)
                        .unwrap_or_default();
                    Ok(Some(GetByNameQueryResult::Note(NoteDetail {
                        name: n.name,
                        topic: n.topic,
                        content: n.content,
                        notebook_name: n.notebook_name,
                        collection_name,
                        tags,
                    })))
                }
                None => Ok(None),
            }
        }
        EntityKind::Tag => {
            let row = tag::Entity::find()
                .filter(tag::Column::Tag.eq(name))
                .one(db)
                .await?;
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
        let GetAllQueryResult::Collections(collections) = result else {
            panic!("expected Collections")
        };
        assert!(!collections.is_empty());
        assert!(collections.iter().any(|c| c.name == data.collection.name));

        // Notebooks
        let result = get_all(&db, EntityKind::Notebook).await.unwrap();
        let GetAllQueryResult::Notebooks(notebooks) = result else {
            panic!("expected Notebooks")
        };
        assert!(!notebooks.is_empty());
        assert!(notebooks.iter().any(|n| n.name == data.notebook.name));

        // Notes (now returns NoteSummary)
        let result = get_all(&db, EntityKind::Note).await.unwrap();
        let GetAllQueryResult::Notes(notes) = result else {
            panic!("expected Notes")
        };
        assert!(!notes.is_empty());
        assert!(notes.iter().any(|n| n.name == data.note.name));

        // Tags
        let result = get_all(&db, EntityKind::Tag).await.unwrap();
        let GetAllQueryResult::Tags(tags) = result else {
            panic!("expected Tags")
        };
        assert!(!tags.is_empty());
        assert!(
            tags.iter()
                .any(|t| t.tag == data.tag.tag)
        );

        testutils::clear_test_data(&db).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_by_name() {
        let db = testutils::setup_test_db().await.unwrap();
        let data = testutils::insert_test_data(&db).await.unwrap();

        // Collection
        let result = get_by_name(&db, EntityKind::Collection, &data.collection.name)
            .await
            .unwrap();
        let Some(GetByNameQueryResult::Collection(collection)) = result else {
            panic!("expected Collection")
        };
        assert_eq!(collection.name, data.collection.name);

        // Notebook (now returns NotebookDetail with notes)
        let result = get_by_name(&db, EntityKind::Notebook, &data.notebook.name)
            .await
            .unwrap();
        let Some(GetByNameQueryResult::Notebook(notebook)) = result else {
            panic!("expected Notebook")
        };
        assert_eq!(notebook.name, data.notebook.name);
        assert!(!notebook.notes.is_empty());
        assert!(notebook.notes.iter().any(|n| n.name == data.note.name));

        // Note (now returns NoteDetail with tags and collection)
        let result = get_by_name(&db, EntityKind::Note, &data.note.name)
            .await
            .unwrap();
        let Some(GetByNameQueryResult::Note(note)) = result else {
            panic!("expected Note")
        };
        assert_eq!(note.name, data.note.name);
        assert_eq!(note.collection_name, data.collection.name);
        assert!(!note.tags.is_empty());
        assert!(note.tags.iter().any(|t| *t == data.tag.tag));

        // Tag
        let result = get_by_name(&db, EntityKind::Tag, &data.tag.tag.to_value())
            .await
            .unwrap();
        let Some(GetByNameQueryResult::Tag(tag)) = result else {
            panic!("expected Tag")
        };
        assert_eq!(tag.tag, data.tag.tag);

        testutils::clear_test_data(&db).await.unwrap();
    }
}
