use crate::{dtos::create_group_dto::{CreateGroupParamsDto, CreateGroupResponseDto}, repositories::group_repository::GroupRepository};

pub async fn execute(CreateGroupParamsDto { name, description, user_ids}: &CreateGroupParamsDto ) -> CreateGroupResponseDto {
    let group_repository = GroupRepository::new().await;

    match group_repository.create(&CreateGroupParamsDto {
        name: name.to_string(),
        description: description.as_ref().map(|desc| desc.clone()),
        user_ids: user_ids.to_vec()
    }).await {
        Ok(user) => CreateGroupResponseDto { id: user.inserted_id.to_string(), name: name.to_string(), email: "".to_string() },
        Err(err) => {
            println!("Error creating user");
            eprint!("{}", err);
            CreateGroupResponseDto { id: "".to_string(), name: "".to_string(), email: "".to_string() }
        }
    }
}