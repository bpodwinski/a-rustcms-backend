UPDATE posts
SET title = $2,
    content = $3,
    status = $4,
    date_published = $5
WHERE id = $1;