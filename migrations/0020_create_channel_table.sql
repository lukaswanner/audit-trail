-- Add migration script here
-- Create table channel
CREATE TABLE channel (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    project_id INTEGER NOT NULL,
	enable_notifications BOOLEAN NOT NULL DEFAULT TRUE,
    FOREIGN KEY (project_id) REFERENCES project (id) ON DELETE CASCADE,
    UNIQUE (title, project_id)
);
