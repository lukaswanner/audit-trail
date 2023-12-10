-- Add migration script here
-- add single event for local testing
INSERT INTO event(icon, title, channel_id, user_id) VALUES ('😴', 'new signup', 1, 1);
INSERT INTO event(icon, title, channel_id, user_id) VALUES ('😎', 'new login', 2, 1);
INSERT INTO event(icon, title, channel_id, user_id) VALUES ('🤑', 'shoes bought', 3, 1);
