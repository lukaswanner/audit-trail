-- Add migration script here
-- Create table channel
CREATE TABLE channel (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    project_id INTEGER NOT NULL,
    FOREIGN KEY (project_id) REFERENCES project (id) ON DELETE CASCADE
);
