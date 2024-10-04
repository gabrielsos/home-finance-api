use argon2::{
    password_hash::{
        rand_core::OsRng,
        SaltString
    },
    PasswordHasher,
    Argon2
};

use crate::{dtos::create_user_dto::{CreateUserParamsDto, CreateUserResponseDto}, repositories::create_user_repository::UserRepository};

pub async fn execute(CreateUserParamsDto { name, email, password}: &CreateUserParamsDto ) -> CreateUserResponseDto {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_e) => return CreateUserResponseDto { id: "".to_string(), name: "".to_string(), email: "".to_string() },
    };

    let user_repository = UserRepository::new().await;

    match user_repository.create_user(&CreateUserParamsDto {
        name: name.to_string(),
        email: email.to_string(),
        password: password_hash
    }).await {
        Ok(user) => CreateUserResponseDto { id: user.inserted_id.to_string(), name: name.to_string(), email: email.to_string() },
        Err(err) => {
            println!("Error creating user");
            eprint!("{}", err);
            CreateUserResponseDto { id: "".to_string(), name: "".to_string(), email: "".to_string() }
        }
    }
}