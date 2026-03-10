// insert sample data into the database
use crate::models::{collection, note, note_tag, notebook, tag, taxonomy};
use sea_orm::{ActiveEnum, ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

pub async fn insert_sample_data(db: &DatabaseConnection) -> anyhow::Result<()> {
    let new_collection_1 = collection::ActiveModel {
        name: Set("My Collection 1".to_string()),
        description: Set("A collection of study notes 1".to_string()),
        ..Default::default()
    };

    let new_collection_2 = collection::ActiveModel {
        name: Set("My Collection 2".to_string()),
        description: Set("A collection of study notes 2".to_string()),
        ..Default::default()
    };

    let new_collection_3 = collection::ActiveModel {
        name: Set("My Collection 3".to_string()),
        description: Set("A collection of study notes 3".to_string()),
        ..Default::default()
    };

    new_collection_1.insert(db).await?;
    new_collection_2.insert(db).await?;
    new_collection_3.insert(db).await?;

    let new_notebook_1 = notebook::ActiveModel {
        collection_name: Set("My Collection 1".to_string()),
        name: Set("My Notebook 1".to_string()),
        description: Set("A notebook for my study notes 1".to_string()),
        ..Default::default()
    };

    let new_notebook_2 = notebook::ActiveModel {
        collection_name: Set("My Collection 2".to_string()),
        name: Set("My Notebook 2".to_string()),
        description: Set("A notebook for my study notes 2".to_string()),
        ..Default::default()
    };

    let new_notebook_3 = notebook::ActiveModel {
        collection_name: Set("My Collection 3".to_string()),
        name: Set("My Notebook 3".to_string()),
        description: Set("A notebook for my study notes 3".to_string()),
        ..Default::default()
    };

    new_notebook_1.insert(db).await?;
    new_notebook_2.insert(db).await?;
    new_notebook_3.insert(db).await?;

    let new_note_1 = note::ActiveModel {
        notebook_name: Set("My Notebook 1".to_string()),
        name: Set("My First Note 1".to_string()),
        topic: Set("General".to_string()),
        content: Set("This is the content of my first note 1.".to_string()),
        ..Default::default()
    };

    let new_note_2 = note::ActiveModel {
        notebook_name: Set("My Notebook 2".to_string()),
        name: Set("My First Note 2".to_string()),
        topic: Set("General".to_string()),
        content: Set("This is the content of my first note 2.".to_string()),
        ..Default::default()
    };

    let new_note_3 = note::ActiveModel {
        notebook_name: Set("My Notebook 3".to_string()),
        name: Set("My First Note 3".to_string()),
        topic: Set("General".to_string()),
        content: Set("This is the content of my first note 3.".to_string()),
        ..Default::default()
    };

    new_note_1.insert(db).await?;
    new_note_2.insert(db).await?;
    new_note_3.insert(db).await?;

    // Insert the tag once (it's a unique enum value)
    let new_tag = tag::ActiveModel {
        tag: Set(taxonomy::Tag::Important),
        ..Default::default()
    };
    new_tag.insert(db).await?;

    // Associate the tag with each note via the note_tag join table
    let note_tag_1 = note_tag::ActiveModel {
        note_name: Set("My First Note 1".to_string()),
        tag_name: Set(taxonomy::Tag::Important.to_value()),
        ..Default::default()
    };
    let note_tag_2 = note_tag::ActiveModel {
        note_name: Set("My First Note 2".to_string()),
        tag_name: Set(taxonomy::Tag::Important.to_value()),
        ..Default::default()
    };
    let note_tag_3 = note_tag::ActiveModel {
        note_name: Set("My First Note 3".to_string()),
        tag_name: Set(taxonomy::Tag::Important.to_value()),
        ..Default::default()
    };

    note_tag_1.insert(db).await?;
    note_tag_2.insert(db).await?;
    note_tag_3.insert(db).await?;

    Ok(())
}

// remove all sample data from the database
pub async fn remove_sample_data(db: &DatabaseConnection) -> anyhow::Result<()> {
    // delete all note_tag associations
    note_tag::Entity::delete_many().exec(db).await?;
    // delete all tags
    tag::Entity::delete_many().exec(db).await?;
    // delete all notes
    note::Entity::delete_many().exec(db).await?;
    // delete all notebooks
    notebook::Entity::delete_many().exec(db).await?;
    // delete all collections
    collection::Entity::delete_many().exec(db).await?;

    Ok(())
}
