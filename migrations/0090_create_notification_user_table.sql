
CREATE TABLE notification_user (
    id SERIAL PRIMARY KEY,
    channel_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    phone_number VARCHAR(20) NOT NULL,
    
    FOREIGN KEY (channel_id) REFERENCES channel (id) ON DELETE CASCADE,
    UNIQUE (channel_id, phone_number)
);
