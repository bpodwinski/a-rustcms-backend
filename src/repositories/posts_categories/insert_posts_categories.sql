INSERT INTO posts_categories (post_id, category_id)
VALUES ($1, $2)
RETURNING id,
    post_id,
    category_id,
    date_created;