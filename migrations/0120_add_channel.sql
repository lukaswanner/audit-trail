-- Add migration script here
-- add single channel for local testing
INSERT INTO channel(title, project_id) VALUES ('signup', 1);
INSERT INTO channel(title, project_id) VALUES ('login', 1);
INSERT INTO channel(title, project_id) VALUES ('logout', 1);

INSERT INTO channel(title, project_id) VALUES ('lights', 2);
