-- Add up migration script here

-- users data
INSERT INTO users (name, password, is_admin)
VALUES ('admin', 'admin', true);
