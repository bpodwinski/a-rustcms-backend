SELECT id,
    title,
    content,
    author_id,
    status::text AS "status!: Status",
    date_published,
    date_created
FROM posts;