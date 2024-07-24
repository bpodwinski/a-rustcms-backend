-- Add up migration script here
ALTER TABLE posts
ADD COLUMN date_published TIMESTAMP;