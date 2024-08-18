INSERT INTO posts_categories (post_id, categories_id)
VALUES ($1, $2)
RETURNING id,
    post_id,
    categories_id,
    date_created;