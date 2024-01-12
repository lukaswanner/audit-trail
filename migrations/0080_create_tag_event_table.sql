-- Create tag event table

CREATE TABLE tag_event (
    id SERIAL PRIMARY KEY,
    tag_id INTEGER NOT NULL,
    event_id INTEGER NOT NULL,
    FOREIGN KEY (tag_id) REFERENCES tag (id) ON DELETE CASCADE,
    FOREIGN KEY (event_id) REFERENCES event (id) ON DELETE CASCADE
);
