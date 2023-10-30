use crate::{
    configs::settings::config_cors,
    daos::dao_container,
    services::auth::{find_by_id, signup},
    ServerConfig,
};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

pub async fn run_server(config: ServerConfig) -> std::io::Result<()> {
    let dao = dao_container::load(config.db);

    let server = HttpServer::new(move || {
        let cors = config_cors();

        App::new()
            .wrap(cors)
            .app_data(dao.signup_data.clone())
            .service(
                web::scope(&config.api_endpoint)
                    .service(signup)
                    .service(find_by_id),
            )
            .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", config.port))?
    .run();

    println!("🚀 Server running at port {}", config.port);

    server.await?;

    Ok(())
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("Joy in every hello! 👋😊")
}
