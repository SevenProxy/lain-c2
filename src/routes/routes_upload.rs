use actix_web::Scope;

pub struct RoutesUpload;


impl RoutesUpload {
    pub fn get(&self) -> Scope {
        web::scope("/api/v1")
            .route("/ping", web::get().to(
                move | req: HttpRequest | {
                
                }
            ))
    }   
}
