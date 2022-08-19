-- Add up migration script here
CREATE TABLE IF NOT EXISTS funders (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    company_id BIGINT REFERENCES companies(id)
);
