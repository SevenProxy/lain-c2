use actix_web::{
    Scope,
    web::Data,
};
use sea_orm::DatabaseConnection;

use std::sync::Arc;

use crate::{
    adapter::RouterCreate,
    application::{
        UploadUseCase,
        UserUseCase
    },
    controllers::UploadController,
    database::{
        UploadRepositoryImpl,
        UserRepositoryImpl
    }
};

pub struct RoutesUpload;


impl RoutesUpload {
    pub fn get(&self, db: &Data<DatabaseConnection>) -> Scope {
        let user_repo: UserRepositoryImpl = UserRepositoryImpl { db: db.get_ref().clone() };
        let user_app: Arc<UserUseCase<UserRepositoryImpl>> = Arc::new(UserUseCase::new(user_repo));

        let upload_repo: UploadRepositoryImpl = UploadRepositoryImpl { db: db.get_ref().clone() };
        let upload_app: Arc<UploadUseCase<UploadRepositoryImpl>> = Arc::new(UploadUseCase::new(upload_repo));

        RouterCreate::new("/api/v1", upload_app.clone(), user_app.clone())
            .api_get("/ping", UploadController::hello)
            .api_get("/upload/{user_id}/{filename}", UploadController::get_file)
            .build()
    }

    pub fn post(&self, db: &Data<DatabaseConnection>) -> Scope {
        let user_repo: UserRepositoryImpl = UserRepositoryImpl { db: db.get_ref().clone() };
        let user_app: Arc<UserUseCase<UserRepositoryImpl>> = Arc::new(UserUseCase::new(user_repo));

        let upload_repo: UploadRepositoryImpl = UploadRepositoryImpl { db: db.get_ref().clone() };
        let upload_app: Arc<UploadUseCase<UploadRepositoryImpl>> = Arc::new(UploadUseCase::new(upload_repo));
        
        RouterCreate::new("/api/v1", upload_app.clone(), user_app.clone())
            .api_upload("/user/upload", UploadController::user_upload)
            .build()
    }
}
