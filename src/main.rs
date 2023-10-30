mod app;
mod configs;
mod constants;

mod daos;
mod models;
mod services;

use configs::{database::Db, settings::load_config};
use mongodb::Database;
use std::sync::Arc;

pub struct ServerConfig {
    pub port: u16,
    pub api_endpoint: String,
    pub db: Arc<Database>,
}

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

    app::run_server(ServerConfig {
        port: config.app_port,
        api_endpoint: config.api_endpoint,
        db: db.instance.clone(),
    })
    .await?;

    Ok(())
}
