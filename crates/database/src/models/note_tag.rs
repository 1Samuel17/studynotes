use sea_orm::entity::prelude::*;

// NoteTag entity representing the many-to-many relationship between notes and tags
#[sea_orm::model]
#[derive(DeriveEntityModel, Clone, Debug, PartialEq, Eq)]
#[sea_orm(table_name = "note_tag")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub note_name: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag_name: String,
    #[sea_orm(belongs_to, from = "note_name", to = "name")]
    pub note: Option<super::note::Entity>,
    #[sea_orm(belongs_to, from = "tag_name", to = "tag")]
    pub tag: Option<super::tag::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
