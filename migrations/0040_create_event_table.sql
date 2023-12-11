-- Add migration script here
-- create event table
CREATE TABLE event (
    id SERIAL PRIMARY KEY,
    icon TEXT NOT NULL,
    title TEXT NOT NULL,
    channel_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    ts TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (channel_id) REFERENCES channel (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES event_user (id) ON DELETE CASCADE
);
