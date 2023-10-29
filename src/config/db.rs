use mongodb::{Client, Database};
use std::sync::Arc;

pub struct Db {
    pub client: Arc<Database>,
}

impl Db {
    pub async fn establish_connection(
        uri: String,
        db_name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::with_uri_str(&uri).await?;
        let database = client.database(&db_name);

        Ok(Db {
            client: Arc::new(database),
        })
    }
}
