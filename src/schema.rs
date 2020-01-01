table! {
    ladders (id) {
        id -> Int4,
    }
}

table! {
    players (id) {
        id -> Int4,
        name -> Varchar,
    }
}

// joinable!(players -> ladders (ladder_id));

allow_tables_to_appear_in_same_query!(ladders, players);
