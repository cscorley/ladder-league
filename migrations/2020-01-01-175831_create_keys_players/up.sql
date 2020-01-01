-- Your SQL goes here
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE leagues
(
    id SERIAL PRIMARY KEY
);

CREATE TABLE players
(
    id SERIAL PRIMARY KEY,
    --    league_id INTEGER REFERENCES keys(id) NOT NULL,
    name VARCHAR NOT NULL
);

