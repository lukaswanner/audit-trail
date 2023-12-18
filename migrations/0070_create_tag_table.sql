-- Create tag table

CREATE TABLE tag (
    id SERIAL PRIMARY KEY,
    key TEXT NOT NULL,
    value TEXT NOT NULL
);
