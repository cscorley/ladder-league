use actix_web::Result;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct LadderInfo {
    name: String,
}

#[derive(Deserialize)]
struct PlayerInfo {
    ladder_id: i32,
    name: String,
}

#[derive(Deserialize)]
struct SwapPlayerInfo {
    ladder_id: i32,
    winner_player_id: i32,
    loser_player_id: i32,
}

async fn get_ladder(info: web::Path<u32>) -> Result<String> {
    Ok(format!("Welcome {}!", info))
}

async fn add_ladder(info: web::Json<LadderInfo>) -> Result<String> {
    Ok(format!("Welcome {}!", info.name))
}

async fn get_player(info: web::Path<u32>) -> Result<String> {
    Ok(format!("Welcome {}!", info))
}

async fn update_player(info: web::Json<PlayerInfo>) -> Result<String> {
    Ok(format!(
        "Welcome {} for ladder {}!",
        info.name, info.ladder_id
    ))
}

async fn add_player(info: web::Json<PlayerInfo>) -> Result<String> {
    Ok(format!(
        "Welcome {} for ladder {}!",
        info.name, info.ladder_id
    ))
}

async fn player_leap(info: web::Json<SwapPlayerInfo>) -> Result<String> {
    Ok(format!(
        "Welcome {} {} {}!",
        info.ladder_id, info.winner_player_id, info.loser_player_id
    ))
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

// get ladder -> name, list of players in order
// add ladder -> name
// add player to ladder -> return new player object
// update player rank -> return player object, calculated rank
// get player -> return player object, calculated rank
// rename player

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/ladder/{id}", web::get().to(get_ladder))
            .route("/ladder", web::post().to(add_ladder))
            .route("/player/{id}", web::get().to(get_player))
            .route("/player", web::post().to(add_player))
            .route("/ladder/player-leap", web::get().to(player_leap))
            .route("/player/{id}", web::put().to(update_player))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
