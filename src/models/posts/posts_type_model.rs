use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "status")]
pub enum Status {
    Draft,
    Pending,
    Private,
    Scheduled,
    Published,
}
