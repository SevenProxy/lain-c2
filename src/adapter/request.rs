use std::{ io, net::IpAddr };
use actix_web::{
    http::{
        header::HeaderMap, Method
    },
    HttpRequest,
};

use crate::error::ServerError;

pub struct Request {
    inner: HttpRequest,
}

impl Request {
    pub fn new(inner: HttpRequest) -> Self {
        Self {
            inner,
        }
    }

    pub fn get_ip(&self) -> Result<IpAddr, ServerError> {
        match self.inner.peer_addr() {
            Some(addr) => Ok(addr.ip()),
            None => Err(ServerError::with_context(
                // source
                io::Error::new(io::ErrorKind::Other, "Sem endereço do peer"),
                "IP inválido."
            )),
        }
    }

    pub fn headers(&self) -> &HeaderMap {
        self.inner.headers()
    }

    pub fn method(&self) -> &Method {
        self.inner.method()
    }

    pub fn path(&self) -> &str {
        self.inner.path()
    }
}
