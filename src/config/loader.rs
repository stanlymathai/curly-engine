pub fn load_configuration() -> Result<(u16, String, String), String> {
    dotenv::dotenv().ok();

    let app_port: u16 = std::env::var("APP_PORT")
        .unwrap_or_else(|_| "7878".to_string())
        .parse()
        .map_err(|_| "APP_PORT must be a valid number".to_string())?;

    let db_name = std::env::var("DB_NAME").map_err(|_| "DB_NAME must be set".to_string())?;
    let db_host = std::env::var("DB_HOST").map_err(|_| "DB_HOST must be set".to_string())?;

    let db_user =
        std::env::var("DB_USERNAME").map_err(|_| "DB_USERNAME must be set".to_string())?;
    let db_password =
        std::env::var("DB_PASSWORD").map_err(|_| "DB_PASSWORD must be set".to_string())?;

    let db_connection_string = format!(
        "mongodb+srv://{}:{}@{}.mongodb.net/{}?retryWrites=true&w=majority",
        db_user, db_password, db_host, db_name
    );

    Ok((app_port, db_connection_string, db_name))
}
