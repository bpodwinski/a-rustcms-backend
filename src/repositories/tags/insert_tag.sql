INSERT INTO tags (
        name,
        slug,
        description
    )
VALUES ($1, $2, $3)
RETURNING id;