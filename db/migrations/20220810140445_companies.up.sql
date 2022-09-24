-- Add up migration script here
CREATE TABLE IF NOT EXISTS companies (
    id BIGSERIAL PRIMARY KEY,
    commercial_feature VARCHAR NOT NULL UNIQUE,
    is_working BOOLEAN NOT NULL,
    legal_entity VARCHAR,
    file_number VARCHAR CONSTRAINT file_number_mus_be_digits CHECK (
        file_number ~ '^\d+$'
        OR file_number IS NULL
    ),
    register_number VARCHAR CONSTRAINT register_number CHECK (
        register_number ~ '^\d{9}$'
        OR register_number IS NULL
    ),
    start_date TIMESTAMPTZ,
    stop_date TIMESTAMPTZ,
    general_tax_mission VARCHAR,
    value_tax_mission VARCHAR,
    activity_nature VARCHAR,
    activity_location VARCHAR,
    record_number VARCHAR,
    username VARCHAR UNIQUE,
    password VARCHAR,
    email VARCHAR UNIQUE
);