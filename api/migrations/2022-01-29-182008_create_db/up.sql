-- Your SQL goes here
CREATE TABLE languages (
  id TEXT PRIMARY KEY NOT NULL,
  name TEXT NOT NULL UNIQUE
);

CREATE TABLE snippets (
  id TEXT PRIMARY KEY NOT NULL,
  code TEXT NOT NULL UNIQUE,
  language_id TEXT NOT NULL,
  FOREIGN KEY (language_id) REFERENCES languages(id)
);
