-- Add migration script here
-- add single channel for local testing
INSERT INTO channel(title, project_id) VALUES ('signup', 1);
INSERT INTO channel(title, project_id) VALUES ('login', 1);
INSERT INTO channel(title, project_id) VALUES ('purchase', 1);

INSERT INTO channel(title, project_id) VALUES ('lights', 2);
INSERT INTO channel(title, project_id) VALUES ('temperature', 2);
INSERT INTO channel(title, project_id) VALUES ('co2', 2);
