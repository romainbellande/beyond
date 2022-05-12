use crate::websocket::AppWs;
use actix_web::{web, HttpResponse, Responder, Scope};
use beyond_core::entities::player::{PlayerDto, PlayerWithoutPassword};
use beyond_core::utils::controller::{handle_error, handle_success};

use super::service;
use super::{credentials::Credentials, jwt_service::Jwt};

async fn register(data: web::Data<AppWs>, player_dto: web::Json<PlayerDto>) -> impl Responder {
    let result = service::register(data.db.clone(), player_dto.into_inner()).await;

    match result {
        Ok(player) => handle_success::<PlayerWithoutPassword>(HttpResponse::Created(), player),
        Err(web_error) => handle_error(web_error),
    }
}

async fn login(data: web::Data<AppWs>, credentials: web::Json<Credentials>) -> impl Responder {
    let result = service::login(data.db.clone(), credentials.into_inner()).await;

    match result {
        Ok(jwt) => handle_success::<Jwt>(HttpResponse::Ok(), jwt),
        Err(web_error) => handle_error(web_error),
    }
}

pub fn controller() -> Scope {
    web::scope("/auth")
        .route("/login", web::post().to(login))
        .route("/register", web::post().to(register))
}
