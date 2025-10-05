use crate::{
    adapter::{
        Request,
        Response,
    },
    dto::JsonResponse
};


pub struct UploadController;

impl UploadController {
    pub async fn hello(&self, _req: Request) -> Response where {
        let message_response: JsonResponse = JsonResponse {
            status: true,
            message: Some(String::from("Welcome")),
            data: None,
        };

        Response::ok(message_response)
    }
}
