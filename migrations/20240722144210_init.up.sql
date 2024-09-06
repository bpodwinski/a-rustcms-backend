-- Add up migration script here
DO $$ BEGIN IF NOT EXISTS (
    SELECT 1
    FROM pg_type
    WHERE typname = 'posts_status'
) THEN CREATE TYPE posts_status AS ENUM (
    'Draft',
    'Pending',
    'Private',
    'Scheduled',
    'Published'
);
END IF;
END $$;
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(200) NOT NULL UNIQUE
);
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(200) NOT NULL,
    slug VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    author_id INTEGER NOT NULL REFERENCES users(id),
    status posts_status NOT NULL DEFAULT 'Draft',
    date_published TIMESTAMP NULL,
    date_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    parent_id INTEGER DEFAULT NULL,
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(200) NOT NULL,
    description VARCHAR(1000) DEFAULT NULL,
    date_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
CREATE TABLE posts_categories (
    id SERIAL PRIMARY KEY,
    post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    category_id INTEGER NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    date_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(200) NOT NULL,
    description VARCHAR(1000) DEFAULT NULL,
    date_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);