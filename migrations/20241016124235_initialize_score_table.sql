-- Add migration script here
CREATE TABLE IF NOT EXISTS games
(
    name  TEXT    PRIMARY KEY NOT NULL UNIQUE,
    score INTEGER             NOT NULL        DEFAULT 0
);

INSERT INTO games ([name],[score])
VALUES
    ('Who Is That Pokemon', 0);
