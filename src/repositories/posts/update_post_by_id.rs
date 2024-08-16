use sqlx::PgPool;

pub async fn update_post_by_id(
    pool: &PgPool,
    post_id: i32,
    title: String,
    content: String,
    status: crate::models::posts::posts_type_model::Status,
    date_published: Option<chrono::NaiveDateTime>,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query_file!(
        "src/repositories/posts/update_post_by_id.sql",
        post_id,
        title,
        content,
        status as crate::models::posts::posts_type_model::Status,
        date_published
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
