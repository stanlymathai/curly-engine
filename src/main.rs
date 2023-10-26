mod api;
mod models;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use api::user_api::{create_user, get_user, update_user, get_env};
use repository::mongodb_repo::MongoRepo;
use std::env;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv::dotenv().ok();
    dotenv::dotenv().expect("Failed to read .env file");
    let app_port = 8080;
    let app_host = "0.0.0.0";
    let db_uri = env::var("MONGO_URI").unwrap();
    let db_name = env::var("DB_NAME").unwrap();

    // println!("=== Environment Variables ===");
    // for (key, value) in env::vars() {
    //     println!("{}: {}", key, value);
    // }
    // println!("=============================");



    
    let db = MongoRepo::establish_connection(&db_uri, &db_name).await;
    let db_data = Data::new(db);

    // Set up and run the web server
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_user)
            .service(update_user)
            .service(get_user)
            .service(get_env)
    })
    .bind((app_host, app_port))?
    .run()
    .await
}
