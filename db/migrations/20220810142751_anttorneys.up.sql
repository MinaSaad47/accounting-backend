-- Add up migration script here
CREATE TABLE IF NOT EXISTS anttorneys (
    id BIGSERIAL PRIMARY KEY,
    number VARCHAR NOT NULL,
    company_id BIGINT REFERENCES companies(id)
)
