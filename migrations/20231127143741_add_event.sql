-- Add migration script here
-- add single event for local testing
INSERT INTO event(icon, title, channel_id, user_id) 
VALUES ('😴', 'Test Event', 1, 1);
