SELECT p.id,
    p.title,
    p.content,
    p.slug,
    p.author_id,
    p.status::text AS "status!: PostsStatus",
    p.date_published,
    p.date_created,
    COALESCE(
        json_agg(
            json_build_object(
                'id',
                c.id,
                'name',
                c.name,
                'description',
                c.description
            )
        ) FILTER (
            WHERE c.id IS NOT NULL
        ),
        '[]'
    ) AS categories
FROM posts p
    LEFT JOIN posts_categories pc ON p.id = pc.post_id
    LEFT JOIN categories c ON pc.category_id = c.id
WHERE p.id = $1
GROUP BY p.id;