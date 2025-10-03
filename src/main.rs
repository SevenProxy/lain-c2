mod error;
mod adapter;
mod controllers;
mod dto;


use actix_web::{
    HttpServer,
    App,

    web::Data,
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
    

    HttpServer::new(|| {
        App::new()
            .service()

    })
    .bind("0.0.0.0", PORT)?
    .run()
    .await
}
