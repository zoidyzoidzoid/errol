-- Your SQL goes here

CREATE TABLE github_issues (
  id SERIAL PRIMARY KEY,
  number INTEGER NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  labels TEXT NOT NULL,
  url TEXT NOT NULL
)