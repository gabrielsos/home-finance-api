use crate::{
  dtos::create_user_dto::{CreateUserParamsDto, CreateUserResponseDto},
  errors::service_error::ServiceError,
};
use async_trait::async_trait;

#[async_trait]
pub trait CreateUserService {
  async fn execute<'a>(
    &'a self,
    params: &'a CreateUserParamsDto,
  ) -> Result<CreateUserResponseDto, ServiceError>;
}
