use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, HttpRequest, http::header::ContentType};

#[get("/")]
pub async fn hello_world() -> &'static str{
    "Ola, mundo"
}