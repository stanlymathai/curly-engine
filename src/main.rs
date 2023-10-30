mod app;
mod configs;
mod models;
mod services;

use configs::{database::Db, settings::load_config};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = match load_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not load configuration: {}", e);
            return Err(e.into());
        }
    };

    let db = Db::connect(config.db_uri, config.db_name)
        .await
        .map_err(|e| {
            eprintln!("Error connecting to DB: {}", e);
            e
        })?;

    app::run_server(config.app_port, db.instance.clone()).await?;

    Ok(())
}
