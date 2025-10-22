use async_trait::async_trait;
use sea_orm::DbErr;
use super::entity::{
    self,
    Model,
    ActiveModel,
};


#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: ActiveModel) -> Result<Model, DbErr>;
    async fn find_user(&self, id: i32) -> Result<Option<Model>, DbErr>;
}
