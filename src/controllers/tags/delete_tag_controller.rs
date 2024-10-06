use ntex::web::{self, types::Json, Error, HttpResponse};
use sqlx::PgPool;

use crate::{
    dtos::tag_dto::DeleteTagIdsDTO,
    handlers::convert_anyhow_to_ntex::convert_anyhow_to_ntex,
    services::tags_service::delete_tag_by_id_service,
};

#[utoipa::path(
    delete,
    path = "/tags",
    tag = "Tags",
    request_body = DeleteTagIdsDTO,
    responses(
        (status = 200, description = "Tags deleted", body = i32),
        (status = 400, description = "Validation Error", body = Error),
        (status = 500, description = "Internal Server Error", body = Error)
    )
)]
#[web::delete("/tags/{id}")]
pub async fn delete_tag_controller(
    pool: web::types::State<PgPool>,
    tag_id: Json<DeleteTagIdsDTO>,
) -> Result<HttpResponse, Error> {
    match delete_tag_by_id_service(pool.get_ref(), tag_id.into_inner()).await {
        Ok(deleted_ids) => Ok(HttpResponse::Ok().json(&deleted_ids)),
        Err(e) => Err(convert_anyhow_to_ntex(e)),
    }
}

#[cfg(test)]
mod tests {
    use ntex::http;
    use ntex::web::{self, test};

    use super::*;
    use crate::tests::helpers::setup::setup_test_db;

    #[ntex::test]
    async fn test_delete_tag_success() {
        // Arrange
        let pool = setup_test_db().await;
        let app = test::init_service(
            web::App::new()
                .state(pool.clone())
                .service(delete_tag_controller),
        )
        .await;

        let inserted_tag = sqlx::query!(
            r#"
            INSERT INTO tags (name, slug, description) 
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            "Test Delete Tag",
            "test-delete-tag",
            "Description for test delete tag"
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to insert test data");

        // Act
        let req = test::TestRequest::delete()
            .uri(&format!("/tags/{}", inserted_tag.id))
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);
    }
}
