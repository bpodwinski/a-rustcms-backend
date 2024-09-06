INSERT INTO posts (
        title,
        content,
        slug,
        author_id,
        status,
        date_published
    )
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING id;