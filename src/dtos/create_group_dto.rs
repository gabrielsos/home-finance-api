use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateGroupParamsDto {
  pub name: String,
  pub description: Option<String>,
  pub user_ids: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateGroupResponseDto {
  pub id: String,
  pub name: String,
  pub email: String,
}
