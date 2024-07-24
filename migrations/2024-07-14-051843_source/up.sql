-- Your SQL goes here
CREATE TABLE source (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  url TEXT NOT NULL,
  country TEXT NOT NULL,
  language TEXT NOT NULL
);
