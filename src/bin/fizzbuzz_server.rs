use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        // default log level is info
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    HttpServer::new(|| App::new().service(fizzbuzz_server::fizzbuzz_api))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
