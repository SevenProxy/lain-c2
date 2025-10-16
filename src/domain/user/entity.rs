use sea_orm::entity::prelude::*;


#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub password: String,
    pub avatar: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::domain::upload::Entity")]
    Upload,
}

impl Related<crate::domain::upload::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Upload.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
