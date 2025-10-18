use async_trait::async_trait;
use sea_orm::{
    DatabaseConnection,
    EntityTrait,
    ActiveModelTrait,
    DbErr,
};
use crate::domain::upload::{
    entity,
    UploadRepository,
};


pub struct UploadRepositoryImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl UploadRepository for UploadRepositoryImpl {
    async fn create(&self, data: entity::ActiveModel) -> Result<entity::Model, DbErr> {
        data.insert(&self.db).await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<entity::Model>, DbErr> {
        entity::Entity::find_by_id(id).one(&self.db).await
    }

    async fn list_all(&self) -> Result<Vec<entity::Model>, DbErr> {
        entity::Entity::find().all(&self.db).await
    }
}
