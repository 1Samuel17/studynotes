use crate::models::taxonomy::Tag;
use sea_orm::entity::prelude::*;

// Tag entity representing a tag that can be associated with notes
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(unique, primary_key, auto_increment = false)]
    pub tag: Tag,
    pub note_name: String,
    #[sea_orm(has_many, via = "note_tag")]
    pub note: HasMany<super::note::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
