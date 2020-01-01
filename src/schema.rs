table! {
    ladders (id) {
        id -> Int4,
        updated_at -> Timestamptz
    }
}

table! {
    players (id) {
        id -> Int4,
        name -> Varchar,
        updated_at -> Timestamptz
    }
}

// joinable!(players -> ladders (ladder_id));

allow_tables_to_appear_in_same_query!(ladders, players);
