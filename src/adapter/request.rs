use std::{ io, net::IpAddr };
use actix_web::{
    http::{
        header::HeaderMap, Method
    },
    HttpRequest,
};
use sea_orm::Iden;

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
                String::from("IP inválido.")
            )),
        }
    }

    pub fn query(&self, name: &str) -> Option<String> {
        self.inner.query_string()
            .split('&')
            .find_map(|q: &str| {
                let mut parts = q.split("=");
                if parts.next()? == name {
                    parts.next().map(|v: &str| v.to_string())
                } else {
                    None
                }
            })
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
