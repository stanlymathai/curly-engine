use actix_cors::Cors;
use actix_web::http::header;
use std::env;

pub struct Config {
    pub app_port: u16,
    pub grpc_port: u16,
    pub db_uri: String,
    pub db_name: String,
    pub api_endpoint: String,
}

pub fn load_config() -> Result<Config, String> {
    dotenv::dotenv().ok();

    Ok(Config {
        app_port: get_env("APP_PORT", "8080")?,
        grpc_port: get_env("GRPC_PORT", "7878")?,
        api_endpoint: get_required_env("API_ENDPOINT")?,
        db_name: get_required_env("DB_NAME")?,
        db_uri: construct_db_uri()?,
    })
}

pub fn config_cors() -> Cors {
    let allowed_orgin = get_required_env("CLIENT_URL").expect("CLIENT_URL is not set");
    Cors::default()
        .allowed_origin(&allowed_orgin)
        .allowed_methods(vec!["POST", "GET", "OPTIONS"])
        .allowed_headers(vec![header::CONTENT_TYPE])
        .max_age(3600) // 1 hour
}

fn get_env(key: &str, default: &str) -> Result<u16, String> {
    env::var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse()
        .map_err(|_| format!("{} must be a valid number", key))
}

fn get_required_env(key: &str) -> Result<String, String> {
    env::var(key).map_err(|_| format!("{} must be set", key))
}

fn construct_db_uri() -> Result<String, String> {
    let db_user = get_required_env("DB_USERNAME")?;
    let db_password = get_required_env("DB_PASSWORD")?;
    let db_host = get_required_env("DB_HOST")?;
    let db_name = get_required_env("DB_NAME")?;

    Ok(format!(
        "mongodb+srv://{}:{}@{}.mongodb.net/{}?retryWrites=true&w=majority",
        db_user, db_password, db_host, db_name
    ))
}
