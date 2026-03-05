use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "notebook")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
    pub description: String,
    #[sea_orm(has_many)]
    pub notes: HasMany<super::note::Entity>,
    pub collection_id: Option<i32>,
    #[sea_orm(belongs_to, from = "collection_id", to = "id")]
    pub collection: HasOne<super::collection::Entity>,

}

impl ActiveModelBehavior for ActiveModel {}