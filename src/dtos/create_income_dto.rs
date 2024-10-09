use bson::serde_helpers::chrono_datetime_as_bson_datetime_optional;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateIncomeParamsDto {
  pub group_id: Option<String>,
  pub owner_user_id: String,
  pub user_ids: Vec<String>,
  pub title: String,
  pub date: String,
  pub category: String,
  pub tag: Option<String>,
  pub recurrent: bool,
  pub amount_in_cents: i64,
  #[serde(with = "chrono_datetime_as_bson_datetime_optional")]
  pub end_date: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateIncomeControllerParamsDto {
  pub group_id: Option<String>,
  pub user_ids: Option<Vec<String>>,
  pub title: String,
  pub date: String,
  pub category: String,
  pub tag: Option<String>,
  pub recurrent: bool,
  pub amount_in_cents: i64,
  pub end_date: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateIncomeResponseDto {
  pub id: String,
  pub group_id: Option<String>,
  pub owner_user_id: String,
  pub user_ids: Vec<String>,
  pub title: String,
  pub date: String,
  pub category: String,
  pub tag: Option<String>,
  pub recurrent: bool,
  pub amount_in_cents: i64,
  pub end_date: Option<DateTime<Utc>>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
