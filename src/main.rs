mod api;
mod models;
mod repository;

use actix_web::{web, web::Data, App, HttpResponse, HttpServer, Responder};
use api::user_api::{create_user, get_user, update_user};
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let db = MongoRepo::establish_connection().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
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
