use sea_orm::entity::prelude::*;


#[derive(Debug, Eq, Ord, Clone, PartialOrd, PartialEq,  DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique, indexed)]
    pub username: String,
    #[sea_orm(unique, indexed)]
    pub email: String,
    pub password: String,
    pub salt: String,
    pub status: i16,
    pub permission: i16,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}