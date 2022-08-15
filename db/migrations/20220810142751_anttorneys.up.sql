-- Add up migration script here
CREATE TABLE IF NOT EXISTS anttorneys (
    id VARCHAR PRIMARY KEY,
    number VARCHAR NOT NULL,
    company_id VARCHAR FOREING KEY REFERENCES companies(id)
)
