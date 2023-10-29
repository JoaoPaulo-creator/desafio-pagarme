use actix_web::{web, App, HttpServer, Responder};
use actix_web::dev::Server;

fn index() -> &'static str{
    "Ola, mundo"
}
mod controllers;
use controllers::*;

pub async fn initializer() -> std::io::Result<()> {
    start().await
}