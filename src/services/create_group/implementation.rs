use async_trait::async_trait;

use crate::{dtos::create_group_dto::{CreateGroupParamsDto, CreateGroupResponseDto}, repositories::group_repository::GroupRepository};

use super::trait_definition::CreateGroupService;

pub struct CreateGroupServiceImpl;

#[async_trait]
impl CreateGroupService for CreateGroupServiceImpl {
  async fn execute<'a>(&'a self, CreateGroupParamsDto { name, description, user_ids}: &'a CreateGroupParamsDto ) -> Result<CreateGroupResponseDto, String> {
    let group_repository = GroupRepository::new().await;
  
    match group_repository.create(&CreateGroupParamsDto {
        name: name.to_string(),
        description: description.as_ref().map(|desc| desc.clone()),
        user_ids: user_ids.to_vec()
    }).await {
        Ok(user) => Ok(CreateGroupResponseDto { id: user.inserted_id.to_string(), name: name.to_string(), email: "".to_string() }),
        Err(err) => {
            println!("Error creating user");
            eprint!("{}", err);
            Err("Falha".to_string())
        }
    }
  }
}
