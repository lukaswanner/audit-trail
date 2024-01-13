-- Add migration script here
-- create user table
CREATE TABLE actor (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL,
	project_id INTEGER NOT NULL,
	properties jsonb NOT NULL,
	FOREIGN KEY (project_id) REFERENCES project (id) ON DELETE CASCADE,
	UNIQUE (name, project_id)
);
