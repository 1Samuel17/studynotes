use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: String,
    pub note_id: Option<i32>,
    #[sea_orm(has_many, via = "note_tag")]
    pub note: HasMany<super::note::Entity>,

}

impl ActiveModelBehavior for ActiveModel {}