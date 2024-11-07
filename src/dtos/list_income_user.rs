use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ListIncomeByUserQueryParams {
  pub date: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct ListIncomeByUserParamsDto {
  pub date: Option<DateTime<Utc>>,
  pub user_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IncomeNormalized {
  pub id: String,
  pub group_id: Option<String>,
  pub user_ids: Vec<String>,
  pub title: String,
  pub date: String,
  pub category: String,
  pub tag: Option<String>,
  pub recurrent: bool,
  pub amount_in_cents: i64,
  pub exclude_dates: Vec<String>,
  pub end_date: Option<String>,
  pub created_at: String,
  pub updated_at: String,
  pub deleted_at: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListIncomeByUserResponseDto {
  pub incomes: Vec<IncomeNormalized>,
}
