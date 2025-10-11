use std::io;

use sea_orm::{
    Database, DatabaseConnection, DbErr,
};

use crate::error::ServerError;

pub struct ConnectionPostgres {
    username: String,
    password: String,
    host: String,
    port: String,
    database: String,
}

impl ConnectionPostgres {
    pub fn new(user: &str, pass: &str, host: &str, port: &str, data: &str) -> Self {
        Self {
            username: user.to_string(),
            password: pass.to_string(),
            host: host.to_string(),
            port: port.to_string(),
            database: data.to_string(),
        }
    }

    pub async fn init(&self) -> Result<DatabaseConnection, ServerError> {
        let url_format_connection: String = format!("postgres://{user}:{pass}@{hos}:{port}/{data}",
            user=self.username,
            pass=self.password,
            hos=self.host,
            port=self.port,
            data=self.database);
        let connection_db: Result<DatabaseConnection, DbErr> = Database::connect(&url_format_connection)
            .await;

        match connection_db {
            Ok(c) => Ok(c),
            Err(e) => Err(ServerError::with_context(
                io::Error::new(io::ErrorKind::Other, "Connection Falied"),
                format!("{:?}", e)
            )),
        }
    }
}
