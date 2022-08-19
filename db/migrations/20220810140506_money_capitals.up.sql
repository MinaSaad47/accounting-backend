-- Add up migration script here
CREATE TABLE IF NOT EXISTS money_capitals (
    id BIGSERIAL PRIMARY KEY,
    value DOUBLE PRECISION NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    company_id BIGINT REFERENCES companies(id)
)
