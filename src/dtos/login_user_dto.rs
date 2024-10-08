use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LoginUserParamsDto {
  pub email: String,
  pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginUserResponseDto {
  pub token_type: String,
  pub expires_in: i64,
  pub access_token: String,
}
