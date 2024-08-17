-- Add up migration script here
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    parent_id INTEGER DEFAULT NULL,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(500) DEFAULT NULL,
    date_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);