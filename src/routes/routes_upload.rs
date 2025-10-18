use actix_web::Scope;

use crate::adapter::RouterCreate;
use crate::controllers::UploadController;


pub struct RoutesUpload;


impl RoutesUpload {
    pub fn get(&self) -> Scope {
        RouterCreate::new("/api/v1")
            .api_get("/hello", UploadController::upload)
    }   
}
