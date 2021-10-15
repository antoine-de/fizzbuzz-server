use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        // default log level is info
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    let state = web::Data::new(fizzbuzz_server::model::State::default());
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(fizzbuzz_server::fizzbuzz_api)
            .service(fizzbuzz_server::stats_api)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
