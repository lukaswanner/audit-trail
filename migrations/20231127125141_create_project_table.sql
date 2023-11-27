-- Add migration script here
-- create project table
CREATE TABLE project (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL
);
