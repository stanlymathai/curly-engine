use crate::models::customer_model::Customer;
use mongodb::{error::Error, results::InsertOneResult, Collection, Database};
use std::sync::Arc;

pub struct CustomerRepo {
    collection: Collection<Customer>,
}

impl CustomerRepo {
    pub fn new(db: Arc<Database>) -> Self {
        let collection = db.collection("customer");
        Self { collection }
    }

    pub async fn create_customer(&self, customer: Customer) -> Result<InsertOneResult, Error> {
        match self.collection.insert_one(customer, None).await {
            Ok(insert_result) => Ok(insert_result),
            Err(e) => Err(e),
        }
    }
}
