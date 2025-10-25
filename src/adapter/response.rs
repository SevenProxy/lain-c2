use crate::{
    adapter::Request, dto::JsonResponse
};

use actix_files::NamedFile;
use actix_web::{
    body,
    Responder,
    HttpRequest,
    HttpResponse,
};


pub struct Response {
    pub response: HttpResponse,
}

impl Response {
    pub fn ok(json: JsonResponse) -> Self {
        Self {
            response: HttpResponse::Ok().json(json),
        }
    }

    pub fn not_found(msg: String) -> Self {
        Self {
            response: HttpResponse::NotFound().json(
                JsonResponse {
                    status: false,
                    message: Some(msg),
                    data: None,
                }
            )
        }
    }

    pub fn bad_request(msg: String) -> Self {
        Self {
            response: HttpResponse::BadRequest().json(JsonResponse {
                status: false,
                message: Some(msg),
                data: None,
            }),
        }
    }

    pub fn internal_error(msg: String) -> Self {
        Self {
            response: HttpResponse::InternalServerError().json(JsonResponse {
                status: false,
                message: Some(msg),
                data: None,
            }),
        }
    }

    pub fn file(req: &HttpRequest, file: NamedFile) -> Self {
        Self {
            response: file.into_response(&req),
        }
    }

    pub fn into_inner(self) -> HttpResponse {
        self.response
    }

}


impl Responder for Response {
    type Body = body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        self.response
    }
}
