-- Add up migration script here
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