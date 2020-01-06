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
    id: i32,
}

impl Responder for PlayerInfo {
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

pub async fn get_player(info: web::Path<(i32,)>, pool: web::Data<Pool>) -> impl Responder {
    let conn = &pool.get().unwrap();

    let results = Player::by_id(conn, info.0);

    PlayerInfo {
        id: info.0,
        ladder_id: info.0,
        name: "you".to_string(),
    }
}

pub async fn update_player(info: web::Json<PlayerInfo>) -> impl Responder {
    PlayerInfo {
        id: info.id,
        ladder_id: info.ladder_id,
        name: info.name.to_string(),
    }
}

pub async fn add_player(info: web::Json<PlayerInfo>) -> impl Responder {
    PlayerInfo {
        id: info.id,
        ladder_id: info.ladder_id,
        name: info.name.to_string(),
    }
}
