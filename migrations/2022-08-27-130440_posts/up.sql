-- Your SQL goes here
CREATE TABLE Posts (
    SLUG            TEXT NOT NULL,
    TITLE           VARCHAR(50) NOT NULL,
    DESCRIPTION     VARCHAR(200),
    CONTENT         TEXT,
    DRAFT           BOOL NOT NULL,
    AUTHOR          VARCHAR(100) NOT NULL,
    PRIMARY KEY     (SLUG),
    FOREIGN KEY (AUTHOR) REFERENCES Users(USERNAME)
)