use std::env;

pub fn get_api_url() -> String {
    env::var("API_URL").expect("API_URL must be set")
}

pub fn get_api_port() -> u16 {
    let port_str = env::var("API_PORT").expect("API_PORT must be set");

    match port_str.parse::<u16>() {
        Ok(port) => port,
        Err(_) => panic!("API_PORT must be a valid u16"),
    }
}

pub fn get_secret_key() -> String {
    env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_cors_allowed_url() -> String {
    env::var("CORS_ALLOWED_URL").expect("CORS_ALLOWED_URL must be set")
}

pub fn get_backtrace() -> u16 {
    let backtrace_str =
        env::var("RUST_BACKTRACE").expect("RUST_BACKTRACE must be set");

    match backtrace_str.parse::<u16>() {
        Ok(backtrace) => backtrace,
        Err(_) => panic!("RUST_BACKTRACE must be a valid u16"),
    }
}
