use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateIncomeParamsDto {
  pub group_id: Option<String>,
  pub user_ids: Vec<String>,
  pub title: String,
  pub date: String,
  pub category: String,
  pub tag: Option<String>,
  pub recurrent: bool,
  pub amount_in_cents: i64,
}

#[derive(Deserialize, Serialize)]
pub struct CreateIncomeResponseDto {
  pub id: String,
  pub group_id: Option<String>,
  pub user_ids: Vec<String>,
  pub title: String,
  pub date: String,
  pub category: String,
  pub tag: Option<String>,
  pub recurrent: bool,
  pub amount_in_cents: i64,
  pub created_at: DateTime<Local>,
  pub updated_at: DateTime<Local>,
}
