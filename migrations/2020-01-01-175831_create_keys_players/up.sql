-- Your SQL goes here
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE ladders
(
    id SERIAL PRIMARY KEY,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    name VARCHAR NOT NULL
);

CREATE TABLE players
(
    id SERIAL PRIMARY KEY,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    name VARCHAR NOT NULL,
    ladder_id INTEGER REFERENCES ladders(id) NOT NULL,
    ranking INTEGER NOT NULL
);

SELECT diesel_manage_updated_at('ladders');
SELECT diesel_manage_updated_at('players');
