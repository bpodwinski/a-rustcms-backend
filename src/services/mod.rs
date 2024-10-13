use crate::dtos::pagination_dto::PaginationInfo;

pub mod auth_service;
pub mod categories_service;
pub mod posts_categories_service;
pub mod posts_services;
pub mod tags_service;
pub mod users_service;

/// Calculate pagination information based on the total number of items, current page, and limit.
///
/// # Arguments
///
/// * `total_items` - The total number of items available.
/// * `page` - The current page number.
/// * `limit` - The number of items per page.
///
/// # Returns
///
/// Returns a `PaginationInfo` struct containing details about the pagination,
/// including the total number of pages, current page, and offset for database queries.
pub fn calculate_pagination(
    total_items: i64,
    page: i64,
    limit: i64,
) -> PaginationInfo {
    let total_pages = (total_items as f64 / limit as f64).ceil() as i64;
    let offset = (page - 1) * limit;
    let current_page = if page > total_pages {
        total_pages
    } else {
        page
    };

    PaginationInfo {
        total_items,
        total_pages,
        current_page,
        offset,
    }
}
