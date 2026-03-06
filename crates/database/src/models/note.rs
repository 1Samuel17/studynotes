use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "note")]
pub struct Model {
    #[sea_orm(unique, primary_key, auto_increment = false)]
    pub name: String,
    pub topic: String,
    pub content: String,
    pub notebook_name: String,
    #[sea_orm(belongs_to, from = "notebook_name", to = "name")]
    pub notebook: HasOne<super::notebook::Entity>,
    #[sea_orm(has_many, via = "note_tag")]
    pub tags: HasMany<super::tag::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

