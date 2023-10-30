use crate::{constants, daos::signup_dao::SignupDao, models::signup_model::SignupData};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Error, HttpResponse,
};

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
    repository: Data<SignupDao>,
    id: Path<String>,
) -> Result<HttpResponse, Error> {
    match repository.find_by_identifier(&id.into_inner()).await {
        Ok(Some(data)) => Ok(HttpResponse::Ok().json(data)),

        Ok(None) => Ok(HttpResponse::NotFound().json(constants::DATA_NOT_FOUND)),

        Err(_) => Err(actix_web::error::ErrorInternalServerError(
            constants::INTERNAL_SERVER_ERROR,
        )),
    }
}
