use std::env;

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_cors_allowed_url() -> String {
    env::var("CORS_ALLOWED_URL").expect("CORS_ALLOWED_URL must be set")
}
