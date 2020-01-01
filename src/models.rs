use super::schema::{ladders, players};
use diesel::r2d2;
use diesel::PgConnection;
use diesel::{self, Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Deserialize, Serialize)]
pub struct Player {
    pub id: i32,
    pub name: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
pub struct Ladder {
    pub id: i32,
}

#[derive(Insertable)]
#[table_name = "players"]
pub struct NewPlayer {
    // pub ladder_id: i32,
    pub name: String,
}

// type alias to use in multiple places
pub type Pool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
