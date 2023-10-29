mod app;

#[actix_web::main]
async fn main() {
    app::initializer().await.expect("TODO: panic message");
}
