-- Your SQL goes here
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE leagues
(
    id SERIAL PRIMARY KEY,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE players
(
    id SERIAL PRIMARY KEY,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    --    league_id INTEGER REFERENCES keys(id) NOT NULL,
    name VARCHAR NOT NULL
);

SELECT diesel_manage_updated_at('leagues');
SELECT diesel_manage_updated_at('players');
