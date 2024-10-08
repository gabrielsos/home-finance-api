use std::env;

use actix_web::{http::header::HeaderMap, Result};
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};

use crate::utils::jwt::Claims;

pub async fn is_valid_jwt(headers: &HeaderMap) -> bool {
  if let Some(auth_header) = headers.get("Authorization") {
    if let Ok(auth_str) = auth_header.to_str() {
      let token = auth_str.trim_start_matches("Bearer ");

      let decoding_key = DecodingKey::from_secret(
        env::var("JWT_SECRET")
          .unwrap_or_else(|_| "".to_string())
          .as_ref(),
      );
      let validation = Validation::new(Algorithm::HS256);
      let token_data: Result<TokenData<Claims>, jsonwebtoken::errors::Error> =
        decode(token, &decoding_key, &validation);

      match token_data {
        Ok(data) => {
          println!("Token válido: {:?}", data.claims);
          return true;
        }
        Err(_) => {
          return false;
        }
      }
    }
  }

  false
}
