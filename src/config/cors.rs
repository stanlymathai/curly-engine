use actix_cors::Cors;
use actix_web::http::header;

pub fn get_cors() -> Cors {
    let allowed_orgin = std::env::var("CLIENT_URL").expect("CLIENT_URL is not set");
    Cors::default()
        .allowed_origin(&allowed_orgin)
        .allowed_methods(vec!["POST"])
        .allowed_headers(vec![header::CONTENT_TYPE])
        .max_age(3600) // 1 hour
}
