use actix_web::http::StatusCode;

use super::service_error::{ReturnValue, ServiceError};

pub struct UnauthorizedError {
  pub service_error: ServiceError,
}

impl UnauthorizedError {
  pub fn new(message: &str) -> Self {
    UnauthorizedError {
      service_error: ServiceError {
        json: ReturnValue {
          message: message.to_string(),
        },
        status_code: StatusCode::from_u16(401).unwrap(),
      },
    }
  }
}
