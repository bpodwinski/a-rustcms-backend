SELECT id,
    parent_id,
    name,
    slug,
    description,
    date_created
FROM categories
WHERE id = $1;