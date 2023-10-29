mod config;
mod models;
mod payload;
mod service;
mod constants;
mod repository;

use actix_web::{web, web::Data, App, HttpResponse, HttpServer, Responder};
use service::signup;
use config::{cors, db::Db, loader};
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


    let server = HttpServer::new(move || {
        let cors_config = cors::get_cors();

        App::new()
            .wrap(cors_config)
            .app_data(repo_data.clone())
            .service(signup)
            .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", app_port))?
    .run();

    println!("🚀 Server running at port {}", app_port);
    server.await?;

    Ok(())
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("Joy in every hello! 👋😊")
}