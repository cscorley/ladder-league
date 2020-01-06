use crate::schema::*;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2;
use diesel::PgConnection;
use diesel::{self, Associations, Identifiable, Insertable, QueryResult, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Deserialize, Serialize)]
pub struct Ladder {
    pub id: i32,
    pub updated_at: DateTime<Utc>,
    pub name: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Deserialize, Serialize)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub updated_at: DateTime<Utc>,
    pub ladder_id: i32,
    pub parent_player_id: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "players"]
pub struct NewPlayer {
    pub ladder_id: i32,
    pub parent_player_id: Option<i32>,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "ladders"]
pub struct NewLadder {
    pub name: String,
}

// type alias to use in multiple places
pub type Pool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

impl Player {
    pub fn by_id(conn: &PgConnection, id: i32) -> QueryResult<Option<Self>> {
        players::table.find(id).first(conn).optional()
    }
}
