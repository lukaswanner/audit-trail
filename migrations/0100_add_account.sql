-- Add migration script here
-- add single project for local testing
INSERT INTO account(email, password) 
VALUES ('lukas.wanner@google.com', 'some-secure-password');
