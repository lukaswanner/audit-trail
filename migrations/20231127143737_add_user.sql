-- Add migration script here
-- add single user for local testing
INSERT INTO event_user(name, properties) VALUES ('Test User', '{"email": "some-email@test.com"}');
