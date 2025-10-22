use actix_web::Scope;

use crate::adapter::RouterCreate;
use crate::controllers::UploadController;


pub struct RoutesUpload;


impl RoutesUpload {
    pub fn get(&self) -> Scope {
        RouterCreate::new("/api/v1")
            .api_get("/ping", UploadController::hello)
    }

    pub fn post(&self) -> Scope {
        RouterCreate::new("/api/v1")
            .api_upload("/user/upload", UploadController::user_upload)
    }
}
