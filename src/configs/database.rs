use mongodb::{bson::doc, Client, Database};
use std::error::Error as StdError;
use std::sync::Arc;

pub struct Db {
    pub instance: Arc<Database>,
}

impl Db {
    pub async fn connect(uri: String, db_name: String) -> Result<Self, Box<dyn StdError>> {
        let client = Client::with_uri_str(&uri).await?;

        let admin_db = client.database("admin");
        let command = doc! {"ping": 1};

        admin_db.run_command(command, None).await.map_err(|e| {
            eprintln!("Failed to ping database: {}", e);
            Box::new(e) as Box<dyn StdError>
        })?;

        let database = client.database(&db_name);

        Ok(Db {
            instance: Arc::new(database),
        })
    }
}
