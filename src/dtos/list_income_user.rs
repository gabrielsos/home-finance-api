use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::entities::income::Income;

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
pub struct ListIncomeByUserResponseDto {
  pub incomes: Vec<Income>,
}
