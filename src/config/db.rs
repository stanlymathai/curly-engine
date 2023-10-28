use mongodb::{Client, Database};
use std::{env, sync::Arc};

pub struct Db {
    pub client: Arc<Database>,
}

impl Db {
    pub async fn establish_connection() -> Self {
        let db_name = env::var("DB_NAME").expect("DB_NAME is not set");
        let db_host = env::var("DB_HOST").expect("DB_HOST is not set");
        let db_user = env::var("DB_USERNAME").expect("DB_USERNAME is not set");
        let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD is not set");

        let uri_str = format!(
            "mongodb+srv://{}:{}@{}.mongodb.net/{}?retryWrites=true&w=majority",
            db_user, db_password, db_host, db_name
        );

        let client = Client::with_uri_str(&uri_str)
            .await
            .expect("Failed to initialize client.");
        let database = client.database(&db_name);

        Db {
            client: Arc::new(database),
        }
    }
}
