use bson::serde_helpers::{
  chrono_datetime_as_bson_datetime, chrono_datetime_as_bson_datetime_optional,
};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Income {
  pub _id: ObjectId,
  pub group_id: Option<ObjectId>,
  pub user_ids: Vec<ObjectId>,
  pub title: String,
  #[serde(with = "chrono_datetime_as_bson_datetime")]
  pub date: DateTime<Utc>,
  pub category: String,
  pub tag: Option<String>,
  pub recurrent: bool,
  pub amount_in_cents: i64,
  pub exclude_dates: Vec<DateTime<Utc>>,
  #[serde(with = "chrono_datetime_as_bson_datetime_optional")]
  pub end_date: Option<DateTime<Utc>>,
  #[serde(with = "chrono_datetime_as_bson_datetime")]
  pub created_at: DateTime<Utc>,
  #[serde(with = "chrono_datetime_as_bson_datetime")]
  pub updated_at: DateTime<Utc>,
  #[serde(with = "chrono_datetime_as_bson_datetime_optional")]
  pub deleted_at: Option<DateTime<Utc>>,
}
