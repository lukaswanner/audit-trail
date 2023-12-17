-- Add migration script here
-- Create insight table
CREATE TABLE insight (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    icon TEXT NOT NULL,
    value TEXT NOT NULL,
    project_id INTEGER NOT NULL,
    FOREIGN KEY (project_id) REFERENCES project (id) ON DELETE CASCADE,
    UNIQUE (title, project_id)
);
