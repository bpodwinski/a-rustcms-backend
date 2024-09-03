SELECT id,
    parent_id,
    name,
    description,
    date_created
FROM categories
WHERE id = $1;