-- This file should undo anything in `up.sql`
ALTER TABLE Posts
DROP tags;
DROP TABLE tags;
DROP INDEX IF EXISTS posts_tag_index;