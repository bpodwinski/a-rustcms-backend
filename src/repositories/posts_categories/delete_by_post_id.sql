DELETE FROM posts_categories
WHERE post_id = $1
RETURNING id,
    post_id,
    category_id,
    date_created;