-- Add migration script here
-- create event table
CREATE TABLE event (
    id SERIAL PRIMARY KEY,
    icon TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    channel_id INTEGER NOT NULL,
    actor_id INTEGER NOT NULL,
    notify BOOLEAN NOT NULL DEFAULT FALSE,
    ts TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (channel_id) REFERENCES channel (id) ON DELETE CASCADE,
    FOREIGN KEY (actor_id) REFERENCES actor (id) ON DELETE CASCADE
);
