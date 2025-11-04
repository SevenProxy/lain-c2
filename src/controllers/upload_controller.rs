use std::{
    path::PathBuf,
    sync::Arc,
};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::mime::Mime;
use tokio::fs::File;
use tokio::fs;
use futures_util::stream::TryStreamExt;
use futures_util::stream::StreamExt;
use tokio::io::AsyncWriteExt;
use actix_web::error;

use crate::application::{
    UploadUseCase,
    UserUseCase,
};
use crate::database::{
    UploadRepositoryImpl,
    UserRepositoryImpl,
};
use crate::{
    adapter::{
        Request,
        Response,
    },
    dto::JsonResponse
};


pub struct UploadController;

impl UploadController {

    pub async fn hello(_req: Request) -> Response {
        let message_response: JsonResponse = JsonResponse {
            status: true,
            message: Some(String::from("Welcome")),
            data: None,
        };

        Response::ok(message_response)
    }

    pub async fn user_upload(req: Request, mut payload: Multipart, upload_case: Arc<UploadUseCase<UploadRepositoryImpl>>, user_case: Arc<UserUseCase<UserRepositoryImpl>>) -> Response {
        
        let user_id = match req.query("user_id") {
            Some(query) => query,
            None => return Response::not_found(String::from("Usuário não existe.")) 
        };

        let number_query = match user_id.parse::<i32>() {
            Ok(query) => query,
            Err(_) => return Response::not_found(String::from("Não foi possível ler ID.")),
        };

        match user_case.find_user(number_query).await { 
            Ok(_dto) => {}, 
            Err(_e) => return Response::not_found(String::from("Usuário não existe."))
        };

        let user_dir: String = format!("src/upload/{}/", sanitize_filename::sanitize(&user_id));
        if let Err(_e) = fs::create_dir_all(&user_dir).await {
            return Response::internal_error(String::from("Não foi possível fazer upload de arquivo."));
        }


        while let Ok(Some(mut field)) = payload.try_next().await {
            if let Some(cd) = field.content_disposition() {
                let filename: String = cd
                    .get_filename()
                    .map(|f: &str| f.to_string())
                    .unwrap_or_else(|| String::from("unnamed_file.bin"));
                
                let filepath = format!("{}{}", user_dir, sanitize_filename::sanitize(&filename));

                let mut file = match File::create(&filepath).await {
                    Ok(f) => f,
                    Err(_err) => {
                        return Response::internal_error(String::from("Error ao criar arquivo."));
                    }
                };

                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(data) => {
                            if let Err(_err) = file.write_all(&data).await {
                                return Response::internal_error(String::from("Error ao ler arquivo."));
                            }
                            if let Some(mime_type) = field.content_type() {
                               let storage_path: String = format!("https://192.158.100.20:3000/api/v1/upload/{}/{}", &user_id, filename);
                                upload_case.create_upload(number_query, filename.clone(), mime_type.essence_str(), storage_path).await;
                            } else {
                                return Response::internal_error(String::from("Não foi possível ler o tipo de arquivo."));
                            }
                        }
                        Err(_err) => {
                            return Response::internal_error(String::from("Error ao ler arquivo."));
                        }
                    }
                }
            }
        }

        Response::ok(
            JsonResponse {
                status: true,
                message: Some(String::from("")),
                data: None,
            }
        )
    }

    pub async fn get_file(req: Request) -> Response {
        let user_id = match req.params("user_id") {
            Some(text) => text,
            None => "none",
        };
        let filename = match req.params("filename") {
            Some(text) => text,
            None => "none",
        };
        
        let file_path: String = format!("src/upload/{}/{}", user_id, filename);
        let path = PathBuf::from(file_path);
        
        if !path.exists() {
            return Response::not_found(String::from("Arquivo não foi encontrado."));
        }

        match NamedFile::open(path) {
            Ok(named_file) => Response::file(&req.get_request(), named_file),
            Err(_e) => Response::internal_error(String::from("Erro ao ler mídia.")),
        }
    }

}
