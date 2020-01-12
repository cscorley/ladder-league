use actix_web::Result;
use actix_web::{web, HttpRequest, Responder};
use actix_web::{Error, HttpResponse};
use futures::future::{ready, Ready};
use serde::Deserialize;
use serde::Serialize;

use crate::models::*;

#[derive(Deserialize, Serialize)]
pub struct PlayerInfo {
    ladder_id: i32,
    name: String,
}

impl Responder for Player {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

impl From<PlayerInfo> for NewPlayer {
    fn from(pp: PlayerInfo) -> Self {
        NewPlayer {
            ladder_id: pp.ladder_id,
            name: pp.name,
            ranking: 1,
        }
    }
}

pub async fn get_player(info: web::Path<(i32,)>, pool: web::Data<Pool>) -> impl Responder {
    let conn = &pool.get().unwrap();

    Player::by_id(conn, info.0)
}

pub async fn add_player(info: web::Json<PlayerInfo>, pool: web::Data<Pool>) -> impl Responder {
    let conn = &pool.get().unwrap();

    Player::add(conn, NewPlayer::from(info.0))
}

#[derive(Deserialize)]
pub struct SwapPlayerInfo {
    winner_player_id: i32,
    loser_player_id: i32,
}

pub async fn player_leap(info: web::Json<SwapPlayerInfo>, pool: web::Data<Pool>) -> impl Responder {
    let conn = &pool.get().unwrap();

    Player::leap(conn, info.winner_player_id, info.loser_player_id).0
}
