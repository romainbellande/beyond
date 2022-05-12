use super::{
    credentials::Credentials,
    jwt_service::{Jwt, JwtService},
};
use actix_web::http;
use beyond_core::{
    entities::{
        player::{Player, PlayerDto, PlayerWithoutPassword},
        AppCollection,
    },
    utils::web_error::WebError,
};
use mongodb::{bson::doc, Database};

pub async fn login(db: Database, credentials: Credentials) -> Result<Jwt, WebError> {
    let player = find_one(db.clone(), credentials.username).await;

    if player.is_none() {
        return Err(WebError {
            code: http::StatusCode::FORBIDDEN,
            message: "wrong username or password".to_string(),
        });
    }

    // TODO: decode player password and check hash

    let jwt_service = JwtService::new();
    let result = jwt_service.create_jwt(player.unwrap().username);

    match result {
        Ok(jwt) => Ok(jwt),
        Err(err) => Err(WebError {
            code: http::StatusCode::INTERNAL_SERVER_ERROR,
            message: err.to_string(),
        }),
    }
}

pub async fn register(
    db: Database,
    player_dto: PlayerDto,
) -> Result<PlayerWithoutPassword, WebError> {
    let collection = db.collection::<Player>(&Player::get_collection_name());

    if is_player_exists(db.clone(), player_dto.clone().username).await {
        return Err(WebError {
            code: http::StatusCode::CONFLICT,
            message: "this username is already taken".to_string(),
        });
    }

    let player = Player::new(player_dto);

    match collection.insert_one(player.clone(), None).await {
        Ok(_) => Ok(player.get()),
        Err(err) => Err(WebError {
            code: http::StatusCode::BAD_REQUEST,
            message: err.to_string(),
        }),
    }
}

async fn find_one(db: Database, username: String) -> Option<Player> {
    let collection = db.collection::<Player>(&Player::get_collection_name());
    let filter = doc! { "username": username };

    match collection.find_one(filter, None).await {
        Ok(result) => result,
        _ => None,
    }
}

async fn is_player_exists(db: Database, username: String) -> bool {
    let result = find_one(db, username).await;
    match result {
        Some(_) => true,
        _ => false,
    }
}
