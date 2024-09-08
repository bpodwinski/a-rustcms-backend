SELECT id,
    name,
    slug,
    description,
    date_created
FROM tags
WHERE id = $1;