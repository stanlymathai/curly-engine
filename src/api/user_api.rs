use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
    use actix_web::{
        post, get, put,
        web::{Data, Json, Path},
        HttpResponse,
    };
    use mongodb::bson::oid::ObjectId;
    use std::env;

    #[post("/user")]
    pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
        let data = User {
            id: None,
            name: new_user.name.to_owned(),
            location: new_user.location.to_owned(),
            title: new_user.title.to_owned(),
        };
        let user_detail = db.create_user(data).await;
        match user_detail {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

    #[get("/env_vars")]
    pub async fn get_env() -> HttpResponse {
        println!("=== Environment Variables init from api request ===");
        for (key, value) in env::vars() {
            println!("{}: {}", key, value);
        }
        println!("=============================");
        let env = std::env::vars().collect::<Vec<(String, String)>>();
        HttpResponse::Ok().json(env)
        // HttpResponse::Ok().body("Check the console for environment variables")
    }

    #[get("/user/{id}")]
    pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
        let id = path.into_inner();
        if id.is_empty() {
            return HttpResponse::BadRequest().body("invalid ID");
        }
        let user_detail = db.get_user(&id).await;
        match user_detail {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

    #[put("/user/{id}")]
    pub async fn update_user(
        db: Data<MongoRepo>,
        path: Path<String>,
        new_user: Json<User>,
    ) -> HttpResponse {
        let id = path.into_inner();
        if id.is_empty() {
            return HttpResponse::BadRequest().body("invalid ID");
        };
        let data = User {
            id: Some(ObjectId::parse_str(&id).unwrap()),
            name: new_user.name.to_owned(),
            location: new_user.location.to_owned(),
            title: new_user.title.to_owned(),
        };
        let update_result = db.update_user(&id, data).await;
        match update_result {
            Ok(update) => {
                if update.matched_count == 1 {
                    let updated_user_info = db.get_user(&id).await;
                    return match updated_user_info {
                        Ok(user) => HttpResponse::Ok().json(user),
                        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                    };
                } else {
                    return HttpResponse::NotFound().body("No user found with specified ID");
                }
            }
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }