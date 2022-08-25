-- Add up migration script here
CREATE TABLE IF NOT EXISTS money_capitals (
    id BIGSERIAL PRIMARY KEY,
    value DOUBLE PRECISION NOT NULL,
    description VARCHAR NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_id BIGINT REFERENCES users(id),
    company_id BIGINT REFERENCES companies(id)
)
