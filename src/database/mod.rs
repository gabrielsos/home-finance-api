// src/database.rs
use mongodb::{Client, Database};
use once_cell::sync::Lazy;
use std::{env, sync::Arc};
use tokio::sync::Mutex;

static DB_INSTANCE: Lazy<Mutex<Option<Arc<Database>>>> =
  Lazy::new(|| Mutex::new(None));

pub async fn init_db() {
  let mut db_instance = DB_INSTANCE.lock().await;
  if db_instance.is_none() {
    let uri = env::var("DATABASE_URL").unwrap_or_else(|_| "".to_string());
    let client = Client::with_uri_str(&uri)
      .await
      .expect("Erro ao conectar ao MongoDB");
    *db_instance = Some(Arc::new(client.database("home-finance")));
  }
}

pub async fn get_db() -> Arc<Database> {
  let db_instance = DB_INSTANCE.lock().await;
  db_instance
    .clone()
    .expect("Banco de dados não inicializado")
}
