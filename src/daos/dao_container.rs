use crate::daos::{customer_dao::CustomerDao, signup_dao::SignupDao};
use actix_web::web;
use mongodb::Database;
use std::sync::Arc;

pub struct DaoContainer {
    pub signup_data: web::Data<SignupDao>,
    pub customer_data: web::Data<CustomerDao>,
}

pub fn load(db_instance: Arc<Database>) -> DaoContainer {
    let signup_repo = SignupDao::new(db_instance.clone());
    let signup_data = web::Data::new(signup_repo);

    let customer_repo = CustomerDao::new(db_instance.clone());
    let customer_data = web::Data::new(customer_repo);

    DaoContainer {
        signup_data,
        customer_data,
    }
}
