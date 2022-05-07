mod config;
mod database;
mod fixtures;
mod planets;
mod websocket;

use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use config::Config;
use database::DatabaseManager;
use dotenvy::dotenv;
use websocket::AppWs;

pub async fn start() -> std::io::Result<()> {
    dotenv().ok();
    setup_logs();

    log::info!("starting HTTP server at http://127.0.0.1:3000");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%r %U [%D ms][%s]"))
            .route("/ws/", web::get().to(index))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let config = Config::new();

    let database_manager = DatabaseManager::new(config.database_url, config.database_name);

    let db = database_manager.start().await;

    fixtures::execute_fixtures(db.clone()).await;

    let resp = ws::start(AppWs { db }, &req, stream);

    println!("{:?}", resp);

    resp
}

pub fn setup_logs() {
    std::env::set_var("RUST_LOG", "actix_web=warn");
    fast_log::init(fast_log::config::Config::new().console()).unwrap();
}
