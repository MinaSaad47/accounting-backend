-- Add up migration script here
CREATE TABLE users (
    id VARCHAR NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);
