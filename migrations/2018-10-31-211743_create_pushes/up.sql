-- Your SQL goes here

CREATE TABLE github_repos (
  id SERIAL PRIMARY KEY,
  repo VARCHAR NOT NULL,
  url TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW(),
  deleted_at TIMESTAMP
);

CREATE TABLE github_pulls (
  github_repo_id INTEGER REFERENCES "github_repos" ("id") DEFERRABLE INITIALLY DEFERRED,
  id SERIAL PRIMARY KEY,
  number INTEGER NOT NULL,
  title VARCHAR NOT NULL,
  repo VARCHAR NOT NULL,
  body TEXT NOT NULL,
  labels TEXT NOT NULL,
  url TEXT NOT NULL,
  data TEXT NOT NULL,
  created_at TIMESTAMP,
  updated_at TIMESTAMP,
  deleted_at TIMESTAMP
);

CREATE TABLE github_issues (
  github_repo_id INTEGER REFERENCES "github_repos" ("id") DEFERRABLE INITIALLY DEFERRED,
  id SERIAL PRIMARY KEY,
  number INTEGER NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  labels TEXT NOT NULL,
  url TEXT NOT NULL,
  data TEXT NOT NULL,
  created_at TIMESTAMP,
  updated_at TIMESTAMP,
  deleted_at TIMESTAMP
);


CREATE TABLE github_pulls_fetches (
  github_repo_id INTEGER REFERENCES "github_repos" ("id") DEFERRABLE INITIALLY DEFERRED,
  id SERIAL PRIMARY KEY,
  repo VARCHAR NOT NULL,
  data TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW(),
  deleted_at TIMESTAMP
);

CREATE TABLE github_issues_fetches (
  github_repo_id INTEGER REFERENCES "github_repos" ("id") DEFERRABLE INITIALLY DEFERRED,
  id SERIAL PRIMARY KEY,
  repo VARCHAR NOT NULL,
  data TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW(),
  deleted_at TIMESTAMP
);

CREATE TABLE rules (
  id INTEGER NOT NULL,
  PRIMARY KEY (id),
  authors TEXT ARRAY,
  branches TEXT ARRAY,
  paths TEXT ARRAY,
  projects TEXT ARRAY,
  reply_to TEXT,
  "to" TEXT ARRAY
);

CREATE INDEX idx_rules_authors ON rules (authors);
CREATE INDEX idx_rules_branches ON rules (branches);
CREATE INDEX idx_rules_projects ON rules (projects);