use crate::services::signup::SignupRepo;
use actix_web::web;
use mongodb::Database;
use std::sync::Arc;

pub struct Repositories {
    pub signup_data: web::Data<SignupRepo>,
}

pub fn load(db_instance: Arc<Database>) -> Repositories {
    let signup_repo = SignupRepo::new(db_instance.clone());
    let signup_data = web::Data::new(signup_repo);

    Repositories { signup_data }
}
