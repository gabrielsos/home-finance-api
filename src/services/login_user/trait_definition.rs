use async_trait::async_trait;

use crate::dtos::login_user_dto::{LoginUserParamsDto, LoginUserResponseDto};

#[async_trait]
pub trait LoginUserService {
  async fn execute<'a>(&'a self, params: &'a LoginUserParamsDto ) -> Result<LoginUserResponseDto, String>;
}