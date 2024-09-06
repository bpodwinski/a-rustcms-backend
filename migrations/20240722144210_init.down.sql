-- Add down migration script here
DO $$ BEGIN IF EXISTS (
    SELECT 1
    FROM pg_type
    WHERE typname = 'posts_status'
) THEN DROP TYPE posts_status;
END IF;
END $$;
DROP TABLE IF EXISTS posts;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS categories;
DROP TABLE IF EXISTS posts_categories;
DROP TABLE IF EXISTS tags;