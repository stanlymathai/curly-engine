use mongodb::{bson::doc, options::FindOptions, Cursor};
use crate::models::signup_model::SignupData;
use mongodb::{error::Error, results::InsertOneResult, Collection, Database};
use futures_util::stream::StreamExt;
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

    pub async fn find_by_identifier(&self, identifier: &str) -> Result<Option<SignupData>, Error> {
        let filter = doc! {
            "identifier": identifier
        };
        let find_options = FindOptions::builder().limit(1).build();
        let mut cursor: Cursor<SignupData> = self.collection.find(filter, find_options).await?;
        
        if let Some(result) = cursor.next().await { 
            match result {
                Ok(data) => return Ok(Some(data)),
                Err(e) => return Err(e),
            }
        }
    
        Ok(None)
    }
}
