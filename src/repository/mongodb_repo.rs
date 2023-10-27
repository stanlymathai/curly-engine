use crate::models::user_model::User;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{InsertOneResult, UpdateResult},
    Client, Collection,
};
use std::env;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn establish_connection() -> Self {
        let db_name = env::var("DB_NAME").unwrap();
        let db_host = env::var("DB_HOST").unwrap();
        let db_user = env::var("DB_USERNAME").unwrap();
        let db_password = env::var("DB_PASSWORD").unwrap();

        let uri_str = format!(
            "mongodb+srv://{}:{}@{}.mongodb.net/?retryWrites=true&w=majority",
            db_user, db_password, db_host
        );

        let client = Client::with_uri_str(uri_str).await.unwrap();
        let db = client.database(&db_name);
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "location": new_user.location,
                    "title": new_user.title
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }
}
