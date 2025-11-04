mod error;
mod adapter;
mod controllers;
mod routes;
mod dto;
mod database;
mod domain;
mod application;

use actix_web::{
    HttpServer,
    App,

    web::Data,
};

use routes::{
    RoutesUpload,
};

use database::ConnectionPostgres;
use sea_orm::DatabaseConnection;

static PORT: u16 = 3000;

pub struct AppState {
    pub app_name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let name_server: Data<AppState> = Data::new(AppState {
        app_name: "Lain Uploads".to_string()
    });
    let route_upload: Data<RoutesUpload> = Data::new(RoutesUpload);
    
    let try_connection_db: DatabaseConnection = ConnectionPostgres::new("littleproblem", "proxy1597", "localhost", "5432", "upload").init()
        .await
        .expect("Error connection");

    let db_conn: Data<DatabaseConnection> = Data::new(try_connection_db);

    HttpServer::new(move || {
        App::new()
            .app_data(name_server.clone())
            .app_data(db_conn.clone())
            .app_data(route_upload.clone())
            .service(
                route_upload.post(&db_conn)
            )
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}
