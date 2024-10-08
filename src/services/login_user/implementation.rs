use argon2::{
  password_hash::{PasswordHash, PasswordVerifier},
  Argon2,
};
use async_trait::async_trait;

use crate::{
  dtos::login_user_dto::{LoginUserParamsDto, LoginUserResponseDto},
  errors::{
    internal_server_error::InternalServerError, service_error::ServiceError,
    unauthorized_error::UnauthorizedError,
  },
  repositories::user_repository::UserRepository,
  utils::jwt::generate_jwt,
};

use super::trait_definition::LoginUserService;

pub struct LoginUserServiceImpl;

#[async_trait]
impl LoginUserService for LoginUserServiceImpl {
  async fn execute<'a>(
    &'a self,
    LoginUserParamsDto { email, password }: &'a LoginUserParamsDto,
  ) -> Result<LoginUserResponseDto, ServiceError> {
    let user_repository = UserRepository::new().await;

    match user_repository.find_user_by_email(email).await {
      Ok(Some(user)) => {
        let parsed_hash =
          PasswordHash::new(&user.password).expect("Error parsing hash");

        if Argon2::default()
          .verify_password(password.as_bytes(), &parsed_hash)
          .is_err()
        {
          return Err(UnauthorizedError::new("Unauthorized").service_error);
        }

        let access_token = generate_jwt(&user._id.to_string());

        return Ok(LoginUserResponseDto {
          access_token,
          expires_in: 3600,
          token_type: "Bearer".to_string(),
        });
      }
      Ok(None) => {
        return Err(UnauthorizedError::new("Unauthorized").service_error);
      }
      Err(err) => {
        println!("Error ");
        eprint!("{}", err);
        return Err(
          InternalServerError::new("Internal server error").service_error,
        );
      }
    }
  }
}
