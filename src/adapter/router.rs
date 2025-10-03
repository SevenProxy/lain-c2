use actix_web::{
    web, HttpRequest, Responder, Scope,
};

use crate::{
    adapter::Request,
};

use std::{
    future::Future, pin::Pin,
};

type HandlerController =
    fn(Request) -> Pin<Box<dyn Future<Output = Box<dyn Responder>> + Send>>;

pub struct RouterCreate {
    scope: Scope,
}

impl RouterCreate {
    pub fn new(scope_name: &str) -> Self {
        Self {
            scope: web::scope(scope_name), 
        }
    }

    pub fn api_get(&self, route_name: &str, controller: HandlerController) -> Scope {
        self.scope.route(route_name, web::get().to(
            move | req: HttpRequest | {
                let request: Request = Request::new(req);
                async move { controller(request).await }
            }
        ))
    }
}
