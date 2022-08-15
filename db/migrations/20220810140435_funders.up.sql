-- Add up migration script here
CREATE TABLE IF NOT EXISTS funders (
    id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    company_id VARCHAR FOREING KEY REFERENCES companies(id)
);
