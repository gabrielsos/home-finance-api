use async_trait::async_trait;

use crate::{
  dtos::create_group_dto::{CreateGroupParamsDto, CreateGroupResponseDto},
  errors::{
    internal_server_error::InternalServerError, service_error::ServiceError,
  },
  repositories::group_repository::GroupRepository,
};

use super::trait_definition::CreateGroupService;

pub struct CreateGroupServiceImpl;

#[async_trait]
impl CreateGroupService for CreateGroupServiceImpl {
  async fn execute<'a>(
    &'a self,
    CreateGroupParamsDto {
      name,
      description,
      user_ids,
    }: &'a CreateGroupParamsDto,
  ) -> Result<CreateGroupResponseDto, ServiceError> {
    let group_repository = GroupRepository::new().await;

    match group_repository
      .create(&CreateGroupParamsDto {
        name: name.to_string(),
        description: description.clone().or(None),
        user_ids: user_ids.to_vec(),
      })
      .await
    {
      Ok(user) => Ok(CreateGroupResponseDto {
        id: user.inserted_id.to_string(),
        name: name.to_string(),
        description: description.clone().or(None),
        user_ids: user_ids.to_vec(),
      }),
      Err(err) => {
        println!("Error creating group");
        eprint!("{}", err);
        Err(InternalServerError::new("Internal server error").service_error)
      }
    }
  }
}
