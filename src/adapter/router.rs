use actix_multipart::Multipart;
use actix_web::{
    web, HttpRequest, Responder, Scope,
};

use crate::{
    adapter::Request,
};

use std::{
    future::Future,
};

pub struct RouterCreate {
    scope: Scope,
}

impl RouterCreate {
    pub fn new(scope_name: &str) -> Self {
        Self {
            scope: web::scope(scope_name), 
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
        F: Fn(Request, Multipart) -> Fut + Clone + 'static,
        Fut: Future<Output = R> + 'static,
        R: Responder + 'static,
    {
        self.scope.route(route_name, web::post().to(
            move | req: HttpRequest, payload: Multipart | {
                let request: Request = Request::new(req);
                let ctrl: F = controller.clone();
                async move { ctrl(request, payload).await }
            }
        ))
    }


}
