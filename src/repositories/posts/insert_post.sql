INSERT INTO posts (
        title,
        content,
        author_id,
        status,
        date_published
    )
VALUES ($1, $2, $3, $4, $5)
RETURNING id;