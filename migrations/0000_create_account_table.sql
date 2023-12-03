-- Add migration script here

-- Create table account
CREATE TABLE account (
	id SERIAL PRIMARY KEY,
	email TEXT UNIQUE NOT NULL,
	password TEXT NOT NULL
);
