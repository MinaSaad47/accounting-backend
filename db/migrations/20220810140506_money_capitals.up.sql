-- Add up migration script here
CREATE TABLE money_capitals (
    value NUMBER NOT NULL,
    company_id VARCHAR FOREING KEY REFERENCES companies(id)
)
