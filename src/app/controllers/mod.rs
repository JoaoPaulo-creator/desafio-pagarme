use actix_web::{App, HttpServer};
mod transaction_controller;

pub async fn start() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .service(transaction_controller::hello_world)
    }).bind(("localhost", 8080))?
        .run().await
}