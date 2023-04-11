-- Your SQL goes here
CREATE TABLE group_events (
    id VARCHAR(255) PRIMARY KEY,
   user_id VARCHAR(255) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    group_id VARCHAR(255) NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    event TEXT NOT NULL,
    message_id VARCHAR(255) REFERENCES messages(id),
    receipt_id VARCHAR(255) REFERENCES receipts(id),
    transaction_id VARCHAR(255) REFERENCES transactions(id),
    created_at BIGINT NOT NULL
);