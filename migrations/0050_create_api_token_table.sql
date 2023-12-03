-- Add migration script here
-- create a table for api tokens that links to projects
CREATE TABLE api_token (
  id SERIAL PRIMARY KEY,
  token text NOT NULL,
  project_id INTEGER REFERENCES project(id) ON DELETE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
