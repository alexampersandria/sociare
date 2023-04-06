-- Your SQL goes here
CREATE TABLE user_sessions (
  id VARCHAR(255) PRIMARY KEY,
  user_id VARCHAR(255) NOT NULL REFERENCES users(id),
  created_at BIGINT NOT NULL,
  accessed_at BIGINT NOT NULL,
  ip_address VARCHAR(255) NOT NULL,
  user_agent TEXT NOT NULL
)