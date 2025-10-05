mod error;
mod adapter;
mod controllers;
mod routes;
mod dto;


use actix_web::{
    HttpServer,
    App,

    web::Data,
};

use routes::{
    RoutesUpload,
};

static PORT: u16 = 3000;

pub struct AppState {
    pub app_name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let name_server: Data<AppState> = Data::new(AppState {
        app_name: "Lain Uploads".to_string()
    });
    

    HttpServer::new(move || {
        App::new()
            .app_data(name_server.clone())
            .service(
                RoutesUpload.get()
            )
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}
