mod auth;
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
use serde::Deserialize;

pub async fn start() -> std::io::Result<()> {
    dotenv().ok();
    setup_logs();

    log::info!("starting HTTP server at http://0.0.0.0:3000");

    let config = Config::new();

    let database_manager = DatabaseManager::new(config.database_url, config.database_name);

    let db = database_manager.start().await;

    fixtures::execute_fixtures(db.clone()).await;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%r %U [%D ms][%s]"))
            .app_data(web::Data::new(AppWs { db: db.clone() }))
            .route("/ws/", web::get().to(index))
            .service(web::scope("api/v1").service(auth::controller::controller()))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

#[derive(Deserialize)]
struct WsAuth {
    jwt: String,
}

async fn index(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppWs>,
    ws_auth: web::Query<WsAuth>
) -> Result<HttpResponse, Error> {
    println!("jwt: {}", ws_auth.jwt);

    let resp = ws::start(
        AppWs {
            db: data.db.clone(),
        },
        &req,
        stream,
    );

    println!("{:?}", resp);

    resp
}

pub fn setup_logs() {
    std::env::set_var("RUST_LOG", "actix_web=warn");
    fast_log::init(fast_log::config::Config::new().console()).unwrap();
}
