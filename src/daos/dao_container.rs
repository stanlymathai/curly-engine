use crate::daos::signup_dao::SignupDao;
use actix_web::web;
use mongodb::Database;
use std::sync::Arc;

pub struct DaoContainer {
    pub signup_data: web::Data<SignupDao>,
}

pub fn load(db_instance: Arc<Database>) -> DaoContainer {
    let signup_repo = SignupDao::new(db_instance.clone());
    let signup_data = web::Data::new(signup_repo);

    DaoContainer { signup_data }
}
