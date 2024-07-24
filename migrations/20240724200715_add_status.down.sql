-- Add down migration script here
ALTER TABLE posts DROP COLUMN status;

DO $$ BEGIN
    DROP TYPE IF EXISTS posts_status;
END $$;
