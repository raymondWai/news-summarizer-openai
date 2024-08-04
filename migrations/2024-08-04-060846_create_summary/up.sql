-- Your SQL goes here
CREATE TABLE summary (
    id SERIAL PRIMARY KEY,
    date TIMESTAMP NOT NULL DEFAULT now(),
    content TEXT NOT NULL
);
CREATE UNIQUE INDEX idx_summary_date ON summary (date);