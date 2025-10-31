use actix_multipart::Multipart;
use actix_web::{
    web,
    HttpRequest, 
    Responder, 
    Scope,
};

use crate::{
    adapter::Request,
    application::{
        UserUseCase,
        UploadUseCase,
    },
    database::{
        UserRepositoryImpl,
        UploadRepositoryImpl,
    }
};

use std::{
    future::Future,
    sync::Arc,
};

pub struct RouterCreate {
    scope: Scope,
    upload_case: Arc<UploadUseCase<UploadRepositoryImpl>>,
    user_case: Arc<UserUseCase<UserRepositoryImpl>>,
}

impl RouterCreate {
    pub fn new(scope_name: &str, upload_case: Arc<UploadUseCase<UploadRepositoryImpl>>, user_case: Arc<UserUseCase<UserRepositoryImpl>>) -> Self {
        Self {
            scope: web::scope(scope_name),
            upload_case,
            user_case,
        }
    }

    pub fn api_get<F, Fut, R>(self, route_name: &str, controller: F) -> Scope 
    where
        F: Fn(Request) -> Fut + Clone + 'static,
        Fut: Future<Output = R> + 'static,
        R: Responder + 'static,
    {
        self.scope.route(route_name, web::get().to(
            move | req: HttpRequest | {
                let request: Request = Request::new(req);
                let ctrl: F = controller.clone();
                async move { ctrl(request).await }
            }
        ))
    }
    

    pub fn api_upload<F, Fut, R>(self, route_name: &str, controller: F) -> Scope
    where 
        F: Fn(Request, Multipart, Arc<UploadUseCase<UploadRepositoryImpl>>, Arc<UserUseCase<UserRepositoryImpl>>) -> Fut + Clone + 'static,
        Fut: Future<Output = R> + 'static,
        R: Responder + 'static,
    {
        self.scope.route(route_name, web::post().to(
            move | req: HttpRequest, payload: Multipart | {
                let request: Request = Request::new(req);
                let ctrl: F = controller.clone();

                let upload_case: Arc<UploadUseCase<UploadRepositoryImpl>> = self.upload_case.clone();
                let user_case: Arc<UserUseCase<UserRepositoryImpl>> = self.user_case.clone();

                async move { ctrl(request, payload, upload_case, user_case).await }
            }
        ))
    }

}
