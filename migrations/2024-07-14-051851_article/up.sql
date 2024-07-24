-- Your SQL goes here
CREATE TABLE article (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  url TEXT NOT NULL,
  keywords TEXT[],
  creator TEXT[],
  video_source TEXT,
  description TEXT NOT NULL,
  content TEXT NOT NULL,
  date TIMESTAMP NOT NULL,
  image_url TEXT,
  source_id SERIAL NOT NULL,
  FOREIGN KEY (source_id) REFERENCES source(id),
  UNIQUE (url),
  language TEXT,
  country TEXT[],
  category TEXT[],
  sentiment TEXT,
  sentiment_stat JSONB
);
