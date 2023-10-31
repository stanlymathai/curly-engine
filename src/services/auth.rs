use crate::{
    constants,
    daos::{customer_dao::CustomerDao, signup_dao::SignupDao},
    models::{
        customer_model::{Customer, SecuritySecret, Status},
        signup_model::SignupData,
    },
};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Error, HttpResponse, Responder, Result,
};

use chrono::Utc;
use uuid::Uuid;

use std::collections::HashMap;
use std::env;

fn get_env_variables() -> HashMap<String, String> {
    env::vars().collect()
}

#[get("/env_vars")]
async fn get_env_vars() -> Result<impl Responder> {
    let env_vars = get_env_variables();
    Ok(HttpResponse::Ok().json(env_vars))
}

#[post("/signup")]
pub async fn signup(
    repository: Data<SignupDao>,
    payload: Json<SignupData>,
) -> Result<HttpResponse, Error> {
    match repository.create(payload.into_inner()).await {
        Ok(_) => Ok(HttpResponse::Created().json(constants::SIGNUP_COMPLETED)),

        Err(_) => Err(actix_web::error::ErrorInternalServerError(
            constants::INTERNAL_SERVER_ERROR,
        )),
    }
}

#[get("/find/{id}")]
pub async fn find_by_id(
    signup_repo: Data<SignupDao>,
    customer_repo: Data<CustomerDao>,
    id: Path<String>,
) -> Result<HttpResponse, Error> {
    match signup_repo.find_by_identifier(&id.into_inner()).await {
        Ok(Some(data)) => {
            let user = data.user_details;

            let customer_doc = Customer {
                id: None,
                name: user.name,
                email: user.email_address,
                signup_ref: data.id.unwrap(),
                date_of_birth: user.date_of_birth,
                residency: user.country_code,
                security_secret: SecuritySecret {
                    key: String::new(),
                    value: String::new(),
                },
                secret_or_key: Uuid::new_v4().to_string(),
                status: Status::Active,
                updated_at: Utc::now(),
                created_at: Utc::now(),
            };

            match customer_repo.create_customer(customer_doc).await {
                Ok(result) => Ok(HttpResponse::Ok().json(result)),

                Err(e) => Err(actix_web::error::ErrorInternalServerError(e.to_string())),
            }
        }

        Ok(None) => Ok(HttpResponse::NotFound().json(constants::DATA_NOT_FOUND)),

        Err(_) => Err(actix_web::error::ErrorInternalServerError(
            constants::INTERNAL_SERVER_ERROR,
        )),
    }
}
