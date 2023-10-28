mod api;
mod config;
mod models;
mod repository;

use actix_web::{web, web::Data, App, HttpResponse, HttpServer, Responder};
use api::user_api::{create_user, get_user, update_user};
use config::{cors, db::Db};
use repository::user_repo::UserRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let app_port: u16 = match std::env::var("APP_PORT")
        .expect("APP_PORT is not set")
        .parse()
    {
        Ok(port) => port,
        Err(_) => panic!("Invalid port number"),
    };

    let db = Db::establish_connection().await;
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
    .bind(("0.0.0.0", app_port))
    .expect("Error binding server to address")
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("Joy in every hello! 👋😊")
}
