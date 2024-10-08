use crate::dtos::create_user_dto::{
  CreateUserParamsDto, CreateUserResponseDto,
};
use async_trait::async_trait;

#[async_trait]
pub trait CreateUserService {
  async fn execute<'a>(
    &'a self,
    params: &'a CreateUserParamsDto,
  ) -> Result<CreateUserResponseDto, String>;
}
