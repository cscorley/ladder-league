use actix_files;
use actix_web::Result;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_web::{Error, HttpResponse};
use diesel::{r2d2::ConnectionManager, r2d2::Pool, PgConnection};
use futures::future::{ready, Ready};
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::path::Path;

#[derive(Deserialize)]
struct LadderInfo {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct PlayerInfo {
    ladder_id: u32,
    name: String,
}

#[derive(Deserialize)]
struct SwapPlayerInfo {
    ladder_id: u32,
    winner_player_id: u32,
    loser_player_id: u32,
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

async fn get_ladder(info: web::Path<(u32,)>) -> Result<String> {
    Ok(format!("Welcome {}!", info.0))
}

async fn add_ladder(info: web::Json<LadderInfo>) -> Result<String> {
    Ok(format!("Welcome {}!", info.name))
}

async fn get_player(info: web::Path<(u32,)>) -> impl Responder {
    PlayerInfo {
        ladder_id: info.0,
        name: "you".to_string(),
    }
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
    dotenv::dotenv().ok();

    let port = env::var("PORT")
        .expect("PORT must be set")
        .parse()
        .expect("PORT must be a number");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let dist_path = Path::new("./dist");
    let manifest_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/dist"));
    let path = if dist_path.exists() {
        dist_path
    } else {
        manifest_path
    };

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .data(pool.clone())
                    .route("/", web::get().to(greet))
                    .route("/{name}", web::get().to(greet))
                    .route("/ladder/{id}", web::get().to(get_ladder))
                    .route("/ladder", web::post().to(add_ladder))
                    .route("/player/{id}", web::get().to(get_player))
                    .route("/player", web::post().to(add_player))
                    .route("/ladder/player-leap", web::get().to(player_leap))
                    .route("/player/{id}", web::put().to(update_player)),
            )
            .service(actix_files::Files::new("/", path).index_file("index.html"))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
