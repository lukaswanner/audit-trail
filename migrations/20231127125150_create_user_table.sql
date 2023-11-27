-- Add migration script here
-- create user table
CREATE TABLE event_user (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    properties jsonb NOT NULL
);
