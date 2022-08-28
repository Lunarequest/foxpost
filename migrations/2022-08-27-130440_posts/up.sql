-- Your SQL goes here
CREATE TABLE Posts (
    SLUG            VARCHAR(100) NOT NULL PRIMARY KEY,
    TITLE           VARCHAR(50) NOT NULL,
    DESCRIPTION     VARCHAR(200),
    CONTENT         TEXT,
    DRAFT           BOOL NOT NULL,
    AUTHOR          VARCHAR(100) NOT NULL,
    FOREIGN KEY (AUTHOR) REFERENCES Users(USERNAME)
);