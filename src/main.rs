mod api;
mod config;
mod models;
mod repository;

use actix_web::{web, web::Data, App, HttpResponse, HttpServer, Responder};
use api::user_api::{create_user, get_user, update_user};
use config::{cors, db::Db, loader};
use repository::user_repo::UserRepo;

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

    let repository = UserRepo::new(db.client.clone());
    let repo_handle = Data::new(repository);

    HttpServer::new(move || {
        let cors_config = cors::get_cors();

        App::new()
            .wrap(cors_config)
            .app_data(repo_handle.clone())
            .service(create_user)
            .service(update_user)
            .service(get_user)
            .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", app_port))?
    .run()
    .await?;

    Ok(())
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("Joy in every hello! 👋😊")
}
