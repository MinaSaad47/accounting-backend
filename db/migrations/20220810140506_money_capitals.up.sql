-- Add up migration script here
CREATE TABLE IF NOT EXISTS money_capitals (
    id VARCHAR PRIMARY KEY,
    value NUMBER NOT NULL,
    time DATE NOT NULL,
    company_id VARCHAR FOREING KEY REFERENCES companies(id)
)
