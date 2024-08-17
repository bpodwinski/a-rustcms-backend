INSERT INTO categories (parent_id, name, description)
VALUES ($1, $2, $3)
RETURNING id;