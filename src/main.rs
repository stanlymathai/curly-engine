mod api;
mod config;
mod models;
mod repository;

use actix_web::{web, web::Data, App, HttpResponse, HttpServer, Responder};

use api::user_api::{create_user, get_user, update_user};
use config::db::Db;
use repository::user_repo::UserRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let db = Db::establish_connection().await;
    let user_repo = UserRepo::new(db.client.clone());
    let user_data = Data::new(user_repo);

    HttpServer::new(move || {
        App::new()
            .app_data(user_data.clone())
            .service(create_user)
            .service(update_user)
            .service(get_user)
            .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("Joy in every hello! 👋😊")
}
