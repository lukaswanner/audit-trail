-- Add migration script here
-- add single user for local testing
INSERT INTO actor(name, project_id, properties) VALUES ('Test User',1, '{"email": "some-email@test.com"}');
