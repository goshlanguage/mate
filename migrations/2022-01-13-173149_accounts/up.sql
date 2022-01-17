-- Your SQL goes here

CREATE TABLE accounts (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  vendor VARCHAR NOT NULL,
  client_key VARCHAR NOT NULL,
  client_secret VARCHAR NOT NULL,
  created TIMESTAMP NOT NULL,
  updated TIMESTAMP
);

CREATE TABLE account_histories (
  id SERIAL PRIMARY KEY,
  account_id  INTEGER NOT NULL,
  balance FLOAT NOT NULL,
  updated TIMESTAMP NOT NULL,
  FOREIGN KEY (account_id) REFERENCES accounts (id) ON DELETE CASCADE
);
