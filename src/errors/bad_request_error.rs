use actix_web::{http::StatusCode, HttpResponse};

use super::service_error::{ReturnValue, ServiceError};

pub struct BadRequestError {
  pub service_error: ServiceError,
}

impl BadRequestError {
  pub fn new(message: &str) -> Self {
    BadRequestError {
      service_error: ServiceError {
        json: ReturnValue {
          message: message.to_string(),
        },
        status_code: StatusCode::from_u16(400).unwrap(),
      },
    }
  }
}
