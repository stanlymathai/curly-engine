use crate::models::signup_model::SignupData;
use mongodb::{error::Error, results::InsertOneResult, Collection, Database};
use std::sync::Arc;

pub struct SignupDao {
    collection: Collection<SignupData>,
}

impl SignupDao {
    pub fn new(db: Arc<Database>) -> Self {
        let collection = db.collection("signup_data");
        Self { collection }
    }

    pub async fn create(&self, data: SignupData) -> Result<InsertOneResult, Error> {
        match self.collection.insert_one(data, None).await {
            Ok(insert_result) => Ok(insert_result),
            Err(e) => Err(e),
        }
    }
    
}
