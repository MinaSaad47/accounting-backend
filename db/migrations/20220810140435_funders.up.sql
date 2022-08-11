-- Add up migration script here
CREATE TABLE funders (
    name VARCHAR NOT NULL,
    company_id VARCHAR FOREING KEY REFERENCES companies(id)
);
