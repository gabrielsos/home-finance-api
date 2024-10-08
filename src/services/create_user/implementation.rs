use async_trait::async_trait;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        SaltString
    },
    PasswordHasher,
    Argon2
};

use crate::{dtos::create_user_dto::{CreateUserParamsDto, CreateUserResponseDto}, repositories::user_repository::UserRepository};

use super::trait_definition::CreateUserService;

pub struct CreateUserServiceImpl;

#[async_trait]
impl CreateUserService for CreateUserServiceImpl {
    async fn execute<'a>(&'a self, CreateUserParamsDto {email, password, name}: &'a CreateUserParamsDto) -> Result<CreateUserResponseDto, String> {
        let salt = SaltString::generate(&mut OsRng);
    
        let argon2 = Argon2::default();
    
        let password_hash = match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(hash) => hash.to_string(),
            Err(_e) => return Err("Falhou".to_string()),
        };
    
        let user_repository = UserRepository::new().await;
    
        match user_repository.create_user(&CreateUserParamsDto {
            name: name.to_string(),
            email: email.to_string(),
            password: password_hash
        }).await {
            Ok(user) => Ok(CreateUserResponseDto { id: user.inserted_id.to_string(), name: name.to_string(), email: email.to_string() }),
            Err(err) => {
                println!("Error creating user");
                eprint!("{}", err);
                Err("Falhou".to_string())
            }
        }
    }
}
