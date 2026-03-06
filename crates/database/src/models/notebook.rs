use sea_orm::entity::prelude::*;

// Notebook entity representing a collection of notes
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "notebook")]
pub struct Model {
    #[sea_orm(unique, primary_key, auto_increment = false)]
    pub name: String,
    pub description: String,
    pub collection_name: String,
    #[sea_orm(belongs_to, from = "collection_name", to = "name")]
    pub collection: HasOne<super::collection::Entity>,
    #[sea_orm(has_many)]
    pub notes: HasMany<super::note::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
