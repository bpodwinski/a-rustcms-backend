use anyhow::Result;
use sqlx::PgPool;

use crate::dtos::post_dto::DeletePostsIdsDTO;

pub async fn delete(
    pool: &PgPool,
    posts_ids: DeletePostsIdsDTO,
) -> Result<DeletePostsIdsDTO, sqlx::Error> {
    if posts_ids.ids.is_empty() {
        return Ok(DeletePostsIdsDTO { ids: vec![] });
    }

    let result = sqlx::query_file!(
        "src/repositories/posts/delete_posts_by_ids.sql",
        &posts_ids.ids
    )
    .fetch_all(pool)
    .await?;

    let deleted_ids: Vec<i32> =
        result.into_iter().map(|record| record.id).collect();

    Ok(DeletePostsIdsDTO { ids: deleted_ids })
}
