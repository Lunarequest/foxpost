-- Your SQL goes here
CREATE TABLE tags (
    tag VARCHAR(20) NOT NULL PRIMARY KEY
);

ALTER TABLE Posts
ADD tags TEXT [] NOT NULL;

CREATE INDEX posts_tag_index ON Posts (tags);