use async_trait::async_trait;
use crate::dtos::create_user_dto::{CreateUserParamsDto, CreateUserResponseDto};

#[async_trait]
pub trait CreateUserService {
    async fn execute<'a>(&'a self, params: &'a CreateUserParamsDto) -> Result<CreateUserResponseDto, String>;
}
