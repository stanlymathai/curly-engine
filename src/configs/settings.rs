use actix_cors::Cors;
use actix_web::http::header;

pub struct Config {
    pub app_port: u16,
    pub db_uri: String,
    pub db_name: String,
    pub api_endpoint: String,
}

pub fn load_config() -> Result<Config, String> {
    dotenv::dotenv().ok();

    let app_port: u16 = std::env::var("APP_PORT")
        .unwrap_or_else(|_| "7878".to_string())
        .parse()
        .map_err(|_| "APP_PORT must be a valid number".to_string())?;

    let api_endpoint =
        std::env::var("API_ENDPOINT").map_err(|_| "API_ENDPOINT must be set".to_string())?;

    let db_name = std::env::var("DB_NAME").map_err(|_| "DB_NAME must be set".to_string())?;
    let db_host = std::env::var("DB_HOST").map_err(|_| "DB_HOST must be set".to_string())?;

    let db_user =
        std::env::var("DB_USERNAME").map_err(|_| "DB_USERNAME must be set".to_string())?;
    let db_password =
        std::env::var("DB_PASSWORD").map_err(|_| "DB_PASSWORD must be set".to_string())?;

    let db_uri = format!(
        "mongodb+srv://{}:{}@{}.mongodb.net/{}?retryWrites=true&w=majority",
        db_user, db_password, db_host, db_name
    );

    Ok(Config {
        app_port,
        db_uri,
        db_name,
        api_endpoint,
    })
}

pub fn config_cors() -> Cors {
    let allowed_orgin = std::env::var("CLIENT_URL").expect("CLIENT_URL is not set");
    Cors::default()
        .allowed_origin(&allowed_orgin)
        .allowed_methods(vec!["POST"])
        .allowed_headers(vec![header::CONTENT_TYPE])
        .max_age(3600) // 1 hour
}
