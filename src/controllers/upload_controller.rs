use crate::{
    adapter::{
        Request,
        Response,
    },
    dto::JsonResponse
};


pub struct UploadController;

impl UploadController {
    pub async fn upload(_req: Request) -> Response {
        let message_response: JsonResponse = JsonResponse {
            status: true,
            message: Some(String::from("Welcome")),
            data: None,
        };

        Response::ok(message_response)
    }
}
