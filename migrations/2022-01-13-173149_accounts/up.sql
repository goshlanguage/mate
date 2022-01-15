-- Your SQL goes here

CREATE TABLE accounts (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  balance FLOAT NOT NULL,
  balance_history FLOAT[] NOT NULL
);

INSERT INTO accounts (name, balance, balance_history) VALUES('test', 0.0, '{0.0}');