use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "collection")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
    pub description: String,
    #[sea_orm(has_many)]
    pub notebooks: HasMany<super::notebook::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}