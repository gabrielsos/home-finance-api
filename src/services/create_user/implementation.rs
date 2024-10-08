use async_trait::async_trait;

use argon2::{
  password_hash::{rand_core::OsRng, SaltString},
  Argon2, PasswordHasher,
};

use crate::{
  dtos::create_user_dto::{CreateUserParamsDto, CreateUserResponseDto},
  errors::{
    bad_request_error::BadRequestError,
    internal_server_error::InternalServerError, service_error::ServiceError,
  },
  repositories::user_repository::UserRepository,
};

use super::trait_definition::CreateUserService;

pub struct CreateUserServiceImpl;

#[async_trait]
impl CreateUserService for CreateUserServiceImpl {
  async fn execute<'a>(
    &'a self,
    CreateUserParamsDto {
      email,
      password,
      name,
    }: &'a CreateUserParamsDto,
  ) -> Result<CreateUserResponseDto, ServiceError> {
    let user_repository = UserRepository::new().await;

    match user_repository.find_user_by_email(&email).await {
      Ok(result) => {
        if result.is_some() {
          return Err(
            BadRequestError::new("User is already registered").service_error,
          );
        }
      }
      Err(_) => {
        return Err(
          InternalServerError::new("Internal server error").service_error,
        );
      }
    }

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = match argon2.hash_password(password.as_bytes(), &salt) {
      Ok(hash) => hash.to_string(),
      Err(_e) => {
        return Err(
          InternalServerError::new("Internal server error").service_error,
        );
      }
    };

    match user_repository
      .create_user(&CreateUserParamsDto {
        name: name.to_string(),
        email: email.to_string(),
        password: password_hash,
      })
      .await
    {
      Ok(user) => Ok(CreateUserResponseDto {
        id: user.inserted_id.to_string(),
        name: name.to_string(),
        email: email.to_string(),
      }),
      Err(err) => {
        println!("Error creating user");
        eprint!("{}", err);
        return Err(
          InternalServerError::new("Internal server error").service_error,
        );
      }
    }
  }
}
