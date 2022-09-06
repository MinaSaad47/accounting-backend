-- Add up migration script here
CREATE TABLE IF NOT EXISTS incomes (
    id BIGSERIAL PRIMARY KEY,
    value DOUBLE PRECISION NOT NULL,
    description VARCHAR NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    admin_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
    company_id BIGINT REFERENCES companies(id) ON DELETE CASCADE
)