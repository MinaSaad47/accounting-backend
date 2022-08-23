-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    is_admin BOOL NOT NULL DEFAULT FALSE,
    value DOUBLE PRECISION NOT NULL
);