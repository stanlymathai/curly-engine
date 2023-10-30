mod app;
mod configs;

use configs::{settings, database::Db};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = match settings::load_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not load configuration: {}", e);
            return Err(e.into());
        }
    };

    let db = Db::establish_connection(config.db_uri, config.db_name)
        .await
        .map_err(|e| {
            eprintln!("Could not establish DB connection: {}", e);
            e
        })?;

    app::run_server(config.app_port, db.instance.clone()).await?;

    Ok(())
}
