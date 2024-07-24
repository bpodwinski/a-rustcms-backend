-- Add up migration script here
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'posts_status') THEN
        CREATE TYPE posts_status AS ENUM ('Draft', 'Pending', 'Private', 'Scheduled', 'Published');
    END IF;
END $$;

ALTER TABLE posts ADD COLUMN status posts_status NOT NULL DEFAULT 'Draft';
