SELECT id,
    title,
    content,
    author_id,
    status::text AS "status!: PostsStatus",
    date_published,
    date_created
FROM posts
WHERE id = $1;