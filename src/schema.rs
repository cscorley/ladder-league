table! {
    leagues (id) {
        id -> Int4,
        updated_at -> Timestamptz,
    }
}

table! {
    players (id) {
        id -> Int4,
        updated_at -> Timestamptz,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    leagues,
    players,
);
