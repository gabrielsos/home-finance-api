use actix_web::http::StatusCode;

use super::service_error::{ReturnValue, ServiceError};

pub struct InternalServerError {
  pub service_error: ServiceError,
}

impl InternalServerError {
  pub fn new(message: &str) -> Self {
    InternalServerError {
      service_error: ServiceError {
        json: ReturnValue {
          message: message.to_string(),
        },
        status_code: StatusCode::from_u16(500).unwrap(),
      },
    }
  }
}
