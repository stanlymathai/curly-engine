mod app;
mod configs;
mod constants;

mod grpc;

mod daos;
mod models;
mod services;

use configs::{database::Db, settings::load_config};
use mongodb::Database;
use std::sync::Arc;

use tokio::{join, spawn};

pub struct HttpServerConfig {
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

    let app_config = HttpServerConfig {
        port: config.app_port,
        api_endpoint: config.api_endpoint,
        db: db.instance.clone(),
    };

    // Spawn both the gRPC and web server's tasks
    let grpc_server = spawn(grpc::serve(config.grpc_port));
    let actix_server = spawn(app::run_server(app_config));

    // Wait for both services to complete, effectively running them
    // concurrently. If any of them fails, the error will be propagated.
    let (grpc_result, actix_result) = join!(grpc_server, actix_server);

    grpc_result??;
    actix_result??;

    Ok(())
}
