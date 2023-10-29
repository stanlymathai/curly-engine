mod app;
mod config;
mod constants;
mod models;
mod payload;
mod repository;
mod service; 

use crate::app::run_server;
use actix_web::web::Data;
use config::{db::Db, loader};
use repository::customer_repo::CustomerRepo;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (app_port, conn_string, db_name) = match loader::load_configuration() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Could not load configuration: {}", e);
            return Err(e.into());
        }
    };

    let db = Db::establish_connection(conn_string, db_name)
        .await
        .map_err(|e| {
            eprintln!("Could not establish DB connection: {}", e);
            e
        })?;

    let repository = CustomerRepo::new(db.instance.clone());
    let repo_data = Data::new(repository);

    run_server(app_port, repo_data).await?;

    Ok(())
}
