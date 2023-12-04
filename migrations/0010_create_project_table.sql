-- Add migration script here
-- create project table
CREATE TABLE project (
    id SERIAL PRIMARY KEY,
    account_id INTEGER NOT NULL,
    title TEXT NOT NULL,

    FOREIGN KEY (account_id) REFERENCES account(id) ON DELETE CASCADE,
    UNIQUE (title, account_id)
);
