#[macro_use]
extern crate diesel;
extern crate chrono;

use actix_files;
use actix_web::{web, App, HttpServer};
use diesel::{r2d2::ConnectionManager, r2d2::Pool, PgConnection};
use std::env;
use std::path::Path;

mod ladder_handler;
mod models;
mod player_handler;
mod schema;

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
                    .route("/ladder/{id}", web::get().to(ladder_handler::get_ladder))
                    .route("/ladder", web::post().to(ladder_handler::add_ladder))
                    .route(
                        "/ladder/player-leap",
                        web::post().to(player_handler::player_leap),
                    )
                    .route("/player/{id}", web::get().to(player_handler::get_player))
                    .route("/player", web::post().to(player_handler::add_player)),
            )
            .service(actix_files::Files::new("/", path).index_file("index.html"))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
