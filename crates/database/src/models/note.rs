use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "note")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: String,
    #[sea_orm(has_many, via = "note_tag")]
    pub tags: HasMany<super::tag::Entity>,
    pub notebook_id: Option<i32>,
    #[sea_orm(belongs_to, from = "notebook_id", to = "id")]
    pub notebook: HasOne<super::notebook::Entity>,

}

impl ActiveModelBehavior for ActiveModel {}