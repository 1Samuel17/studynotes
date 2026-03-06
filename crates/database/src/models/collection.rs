use sea_orm::entity::prelude::*;

// Collection entity representing a group of notebooks
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "collection")]
pub struct Model {
    #[sea_orm(unique, primary_key, auto_increment = false)]
    pub name: String,
    pub description: String,
    #[sea_orm(has_many)]
    pub notebooks: HasMany<super::notebook::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
