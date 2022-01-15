-- Your SQL goes here

CREATE TABLE accounts (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  balance FLOAT NOT NULL,
  balance_history FLOAT[] NOT NULL
)
