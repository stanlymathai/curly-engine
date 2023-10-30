use crate::{
    configs::settings::config_cors,
    daos::dao_container,
    services::signup::signup,
};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mongodb::Database;
use std::sync::Arc;

pub async fn run_server(app_port: u16, db_instance: Arc<Database>) -> std::io::Result<()> {
    let dao = dao_container::load(db_instance);

    let server = HttpServer::new(move || {
        let cors = config_cors();

        App::new()
            .wrap(cors)
            .app_data(dao.signup_data.clone())
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
