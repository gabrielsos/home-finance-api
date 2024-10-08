use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateUserParamsDto {
  pub name: String,
  pub email: String,
  pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserResponseDto {
  pub id: String,
  pub name: String,
  pub email: String,
}

#[derive(Deserialize, Serialize)]
pub enum CreateUserResponseErrorDto {
  BadRequestError,
  InternalServerError,
}
