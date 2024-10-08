use async_trait::async_trait;

use crate::{
  dtos::create_group_dto::{CreateGroupParamsDto, CreateGroupResponseDto},
  errors::service_error::ServiceError,
};

#[async_trait]
pub trait CreateGroupService {
  async fn execute<'a>(
    &'a self,
    params: &'a CreateGroupParamsDto,
  ) -> Result<CreateGroupResponseDto, ServiceError>;
}
