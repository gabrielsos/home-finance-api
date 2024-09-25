use mongodb::{Client, Database};
use std::sync::Arc;
use mongodb::error::Result;

#[derive(Clone)]
pub struct MongoDB {
    pub database: Arc<Database>,
}

impl MongoDB {
    pub async fn init(uri: &str, db_name: &str) -> Result<Self> {
        let client = Client::with_uri_str(uri).await?;
        let database = client.database(db_name);

        Ok(MongoDB {
            database: Arc::new(database),
        })
    }
}
