UPDATE posts
SET title = $2,
    content = $3,
    slug = $4,
    status = $5,
    date_published = $6
WHERE id = $1;