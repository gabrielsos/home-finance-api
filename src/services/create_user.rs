use crate::dtos::create_user_dto::CreateUserDto;

pub async fn execute(CreateUserDto { name, email}: CreateUserDto ) -> CreateUserDto {
    CreateUserDto { name, email }
}