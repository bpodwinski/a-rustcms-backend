-- Add up migration script here
ALTER TABLE posts
ADD COLUMN date_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL;