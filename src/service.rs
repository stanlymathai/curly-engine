use crate::constants;
use crate::payload::signup_payload::SignupPayload;
use crate::{
    models::customer_model::{Customer, SecuritySecret, Status},
    repository::customer_repo::CustomerRepo,
};

use actix_web::{
    post,
    web::{Data, Json},
    Error, HttpResponse,
};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

#[post("/signup")]
pub async fn signup(
    customer_repo: Data<CustomerRepo>,
    payload: Json<SignupPayload>,
) -> Result<HttpResponse, Error> {
    if payload.validate().is_err() {
        return Err(actix_web::error::ErrorBadRequest(constants::BAD_REQUEST));
    }

    let customer_doc = Customer {
        id: None,
        name: payload.name.clone(),
        email: payload.email.clone(),
        date_of_birth: payload.date_of_birth,
        residency: payload.residency.clone(),
        security_secret: SecuritySecret {
            key: String::new(),
            value: String::new(),
        },
        secret_or_key: Uuid::new_v4().to_string(),
        status: Status::Pending,
        updated_at: Utc::now(),
        created_at: Utc::now(),
    };

    match customer_repo.create_customer(customer_doc).await {
        Ok(_) => Ok(HttpResponse::Created().json(constants::CUSTOMER_CREATED)),
        Err(_) => {
            Err(actix_web::error::ErrorInternalServerError(
                constants::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}
