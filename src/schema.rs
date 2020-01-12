table! {
    ladders (id) {
        id -> Int4,
        updated_at -> Timestamptz,
        name -> Varchar,
    }
}

table! {
    players (id) {
        id -> Int4,
        updated_at -> Timestamptz,
        name -> Varchar,
        ladder_id -> Int4,
        ranking -> Int4,
    }
}

joinable!(players -> ladders (ladder_id));

allow_tables_to_appear_in_same_query!(
    ladders,
    players,
);
