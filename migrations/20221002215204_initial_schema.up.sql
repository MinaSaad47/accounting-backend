-- Add up migration script here
-- users table
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL CONSTRAINT user_name_must_be_unique UNIQUE,
    password VARCHAR NOT NULL,
    is_admin BOOL NOT NULL DEFAULT FALSE,
    value DOUBLE PRECISION NOT NULL DEFAULT 0
);
-- companies table
CREATE TABLE IF NOT EXISTS companies (
    id BIGSERIAL PRIMARY KEY,
    owner VARCHAR NOT NULL,
    commercial_feature VARCHAR NOT NULL,
    is_working BOOLEAN NOT NULL,
    legal_entity VARCHAR,
    file_number VARCHAR CONSTRAINT company_file_number_must_be_digits CHECK (
        file_number ~ '^(\d)+$'
        OR file_number IS NULL
    ),
    register_number VARCHAR CONSTRAINT company_register_number_must_be_9_digits CHECK (
        register_number ~ '^\d{9}$'
        OR register_number IS NULL
    ),
    start_date TIMESTAMPTZ,
    stop_date TIMESTAMPTZ,
    general_tax_mission VARCHAR,
    value_tax_mission VARCHAR,
    activity_nature VARCHAR,
    activity_location VARCHAR,
    record_number VARCHAR CONSTRAINT company_record_number_must_be_digits CHECK (
        record_number ~ '^\d+$'
        OR record_number IS NULL
    ),
    username VARCHAR CONSTRAINT company_username_must_be_unique UNIQUE,
    password VARCHAR,
    email VARCHAR CONSTRAINT company_email_must_be_unique UNIQUE,
    CONSTRAINT company_must_be_unique UNIQUE(owner, commercial_feature)
);
-- funders table
CREATE TABLE IF NOT EXISTS funders (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    company_id BIGINT NOT NULL REFERENCES companies(id) ON DELETE CASCADE
);
-- incomes table
CREATE TABLE IF NOT EXISTS incomes (
    id BIGSERIAL PRIMARY KEY,
    value DOUBLE PRECISION NOT NULL,
    description VARCHAR NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    admin_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
    company_id BIGINT REFERENCES companies(id) ON DELETE CASCADE
);
-- expenses table
CREATE TABLE IF NOT EXISTS expenses (
    id BIGSERIAL PRIMARY KEY,
    value DOUBLE PRECISION NOT NULL,
    description VARCHAR NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
    company_id BIGINT REFERENCES companies(id) ON DELETE CASCADE
)