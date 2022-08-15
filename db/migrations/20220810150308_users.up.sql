-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id VARCHAR NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);
