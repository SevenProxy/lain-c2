use crate::domain::upload::{
    UploadRepository,
    entity,
};

use sea_orm::{
    ActiveValue::Set, DbErr, Iden
};

use chrono::Utc;


pub struct UploadUseCase<R: UploadRepository> {
    repo: R,
}

impl<R: UploadRepository> UploadUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self {
            repo,
        }
    }

    pub async fn create_upload(&self, user_id: i32, filename: String, mime_type: &str, storage_path: String) -> Result<entity::Model, DbErr> { 
        let active = entity::ActiveModel { 
            user_id: Set(user_id),
            filename: Set(filename),
            mime_type: Set(mime_type.to_string()),
            storage_path: Set(storage_path),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        self.repo.create(active).await
    }

    pub async fn list_ulploads(&self) -> Result<Vec<entity::Model>, DbErr> {
        self.repo.list_all().await
    }
}
