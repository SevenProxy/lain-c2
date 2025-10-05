use actix_web::{
    web, HttpRequest, Responder, Scope,
};

use crate::{
    adapter::Request,
};

use std::{
    future::Future,
};

type HandlerController<R> =
    fn(Request) -> R;

pub struct RouterCreate {
    scope: Scope,
}

impl RouterCreate {
    pub fn new(scope_name: &str) -> Self {
        Self {
            scope: web::scope(scope_name), 
        }
    }

    pub fn api_get<F, Fut, R>(self, route_name: &str, controller: fn(Request) -> F) -> Scope 
    where
        F: Future<Output = R> + 'static,
        R: Responder + 'static,
    {
        self.scope.route(route_name, web::get().to(
            move | req: HttpRequest | {
                let request: Request = Request::new(req);
                async move { controller(request).await }
            }
        ))
    }
}
