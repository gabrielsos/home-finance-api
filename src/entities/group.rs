use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
  pub _id: ObjectId,
  pub name: String,
  pub description: Option<String>,
  pub user_ids: Vec<String>,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}