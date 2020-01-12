use actix_web::Result;
use actix_web::{web, HttpRequest, Responder};
use actix_web::{Error, HttpResponse};
use futures::future::{ready, Ready};
use serde::Deserialize;
use serde::Serialize;

use crate::models::*;

#[derive(Deserialize, Serialize)]
pub struct LadderInfo {
    name: String,
}

impl Responder for Ladder {
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

impl From<LadderInfo> for NewLadder {
    fn from(pp: LadderInfo) -> Self {
        NewLadder { name: pp.name }
    }
}

pub async fn get_ladder(info: web::Path<(i32,)>, pool: web::Data<Pool>) -> impl Responder {
    let conn = &pool.get().unwrap();

    Ladder::by_id(conn, info.0)
}

pub async fn add_ladder(info: web::Json<LadderInfo>, pool: web::Data<Pool>) -> impl Responder {
    let conn = &pool.get().unwrap();

    Ladder::add(conn, NewLadder::from(info.0))
}
