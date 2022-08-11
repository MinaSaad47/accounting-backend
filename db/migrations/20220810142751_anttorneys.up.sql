-- Add up migration script here
CREATE TABLE anttorneys (
    number VARCHAR NOT NULL,
    company_id VARCHAR FOREING KEY REFERENCES companies(id)
)
