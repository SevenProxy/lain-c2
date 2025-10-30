use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};
use crate::domain::user::{
    entity,
    ActiveModel,
    UserRepository
};



pub struct UserRepositoryImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, data: ActiveModel) -> Result<entity::Model, DbErr> {
        data.insert(&self.db).await
    }

    async fn find_user(&self, id: i32) -> Result<Option<entity::Model>, DbErr> {
        entity::Entity::find_by_id(id).one(&self.db).await
    }
}
