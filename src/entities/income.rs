use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Income {
  pub _id: ObjectId,
  pub group_id: Option<ObjectId>,
  pub user_ids: Vec<ObjectId>,
  pub title: String,
  pub date: DateTime,
  pub category: String,
  pub tag: Option<String>,
  pub recurrent: bool,
  pub amount_in_cents: i64,
  pub created_at: DateTime,
  pub updated_at: DateTime,
  pub deleted_at: Option<DateTime>,
}