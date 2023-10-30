use crate::constants;
use crate::{
    models::signup_model::SignupData,
    daos::signup_dao::SignupDao,
};

use actix_web::{
    post,
    web::{Data, Json},
    Error, HttpResponse,
};

#[post("/signup")]
pub async fn signup(
    repository: Data<SignupDao>,
    payload: Json<SignupData>,
) -> Result<HttpResponse, Error> {


    match repository.create(payload.into_inner()).await {
        Ok(_) => Ok(HttpResponse::Created().json(constants::SIGNUP_COMPLETED)),
        Err(_) => {
            Err(actix_web::error::ErrorInternalServerError(
                constants::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}
