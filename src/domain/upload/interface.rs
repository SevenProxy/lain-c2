use async_trait::async_trait;
use sea_orm::DbErr;
use super::entity::{
    self,
    Model,
    ActiveModel,
};

#[async_trait]
pub trait UploadRepository: Send + Sync {
    async fn create(&self, data: ActiveModel) -> Result<Model, DbErr>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr>;
    async fn list_all(&self) -> Result<Vec<Model>, DbErr>;
}
