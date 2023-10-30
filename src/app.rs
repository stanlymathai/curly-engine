use crate::config::cors;
use crate::handler::signup_handler::signup;
use crate::repository::signup_repo::SignupRepo;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mongodb::Database;
use std::sync::Arc;

pub async fn run_server(app_port: u16, db_instance: Arc<Database>) -> std::io::Result<()> {
    let repository = SignupRepo::new(db_instance);
    let repo_data = web::Data::new(repository);

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
