-- Your SQL goes here
CREATE TABLE users (
  id VARCHAR(255) PRIMARY KEY,
  username VARCHAR(255) NOT NULL,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(319) NOT NULL,
  password VARCHAR(255) NOT NULL,
  created_at BIGINT NOT NULL,
  mobilepay VARCHAR(255),
  paypal_me VARCHAR(255),
  deleted BOOLEAN NOT NULL DEFAULT FALSE
)