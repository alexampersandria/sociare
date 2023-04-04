-- Your SQL goes here
CREATE TABLE groups (
  id VARCHAR(255) PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  emoji VARCHAR(255) NOT NULL,
  currency VARCHAR(255) NOT NULL,
  created_at BIGINT NOT NULL
)