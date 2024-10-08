use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::{
  env,
  time::{SystemTime, UNIX_EPOCH},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
  sub: String, // ID do usuário
  exp: usize,  // Data de expiração
}

pub fn generate_jwt(user_id: &str) -> String {
  let expiration = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
    .as_secs()
    + 3600;

  let claims = Claims {
    sub: user_id.to_string(),
    exp: expiration as usize,
  };

  let header = Header::default();
  let encoding_key = EncodingKey::from_secret(
    env::var("JWT_SECRET")
      .unwrap_or_else(|_| "".to_string())
      .as_ref(),
  );

  match encode(&header, &claims, &encoding_key) {
    Ok(token) => token,
    Err(err) => {
      eprintln!("Erro ao gerar token: {}", err);
      String::new()
    }
  }
}
