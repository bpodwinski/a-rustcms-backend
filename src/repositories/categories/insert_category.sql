INSERT INTO categories (parent_id, name, slug, description)
VALUES ($1, $2, $3, $4)
RETURNING id,
    parent_id,
    name,
    slug,
    description,
    date_created;