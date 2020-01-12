use crate::schema::*;
use chrono::{DateTime, Utc};
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::r2d2;
use diesel::PgConnection;
use diesel::{self, Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Deserialize, Serialize)]
pub struct Ladder {
    pub id: i32,
    pub updated_at: DateTime<Utc>,
    pub name: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Deserialize, Serialize)]
pub struct Player {
    pub id: i32,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub ladder_id: i32,
    pub ranking: i32,
}

#[derive(Insertable)]
#[table_name = "players"]
pub struct NewPlayer {
    pub ladder_id: i32,
    pub name: String,
    pub ranking: i32,
}

#[derive(Insertable)]
#[table_name = "ladders"]
pub struct NewLadder {
    pub name: String,
}

// type alias to use in multiple places
pub type Pool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

impl Player {
    pub fn by_id(conn: &PgConnection, id: i32) -> Option<Self> {
        players::table
            .find(id)
            .first(conn)
            .optional()
            .expect("database connection failed")
    }

    pub fn add(conn: &PgConnection, mut player: NewPlayer) -> Option<Self> {
        use crate::schema::players::dsl::*;

        let mut max_ranking: i64 = players
            .filter(ladder_id.eq(player.ladder_id))
            .select(count_star())
            .first(conn)
            .expect("database connection failed");

        max_ranking += 1;

        player.ranking = max_ranking.try_into().expect("rank too large");

        diesel::insert_into(players)
            .values(player)
            .get_result(conn)
            .optional()
            .expect("database connection failed")
    }

    pub fn leap(
        conn: &PgConnection,
        winner_id: i32,
        loser_id: i32,
    ) -> (Option<Self>, Option<Self>) {
        let winner = Player::by_id(conn, winner_id);
        let loser = Player::by_id(conn, loser_id);

        if winner.is_none() | loser.is_none() {
            // cannot update
            return (winner, loser);
        }

        use crate::schema::players::dsl::*;

        let winner = winner.unwrap();
        let loser = loser.unwrap();

        if winner.ranking < loser.ranking {
            // already top
            return (Some(winner), Some(loser));
        }

        // increment the ranks of everyone after the loser (inclusive), but before the winner
        diesel::update(
            players
                .filter(ladder_id.eq(winner.ladder_id))
                .filter(ranking.ge(loser.ranking))
                .filter(ranking.lt(winner.ranking)),
        )
        .set(ranking.eq(ranking + 1))
        .execute(conn)
        .expect("database connection failed");

        let new_winner = diesel::update(players.find(winner.id))
            .set(ranking.eq(loser.ranking))
            .get_result(conn)
            .optional()
            .expect("database connection failed");

        let new_loser = Player::by_id(conn, loser_id);

        (new_winner, new_loser)
    }
}

impl Ladder {
    pub fn by_id(conn: &PgConnection, id: i32) -> Option<Self> {
        ladders::table
            .find(id)
            .first(conn)
            .optional()
            .expect("database connection failed")
    }

    pub fn add(conn: &PgConnection, ladder: NewLadder) -> Option<Self> {
        use crate::schema::ladders::dsl::*;

        diesel::insert_into(ladders)
            .values(ladder)
            .get_result(conn)
            .optional()
            .expect("database connection failed")
    }
}
