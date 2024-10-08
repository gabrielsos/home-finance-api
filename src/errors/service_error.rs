use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

pub struct ServiceError {
  pub json: ReturnValue,
  pub status_code: StatusCode,
}

#[derive(Serialize)]
pub struct ReturnValue {
  pub message: String,
}
