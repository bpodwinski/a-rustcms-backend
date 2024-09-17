DELETE FROM posts
WHERE id = ANY($1::int []);