use argon2::{
    password_hash::{
        PasswordHash, PasswordVerifier,
    },
    Argon2
};

use crate::{dtos::login_user_dto::{LoginUserParamsDto, LoginUserResponseDto}, repositories::create_user_repository::UserRepository, utils::jwt::generate_jwt};

pub async fn execute(LoginUserParamsDto { email, password}: &LoginUserParamsDto ) -> LoginUserResponseDto {
    let user_repository = UserRepository::new().await;

    match user_repository.find_user_by_email(email).await {
        Ok(Some(user)) => {
            let parsed_hash = PasswordHash::new(&user.password).expect("Error parsing hash");

            if Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_err() {
                return LoginUserResponseDto { access_token: "".to_string(), token_type: "".to_string(), expires_in: 0 };
            }

            let access_token = generate_jwt(&user._id.to_string());

            return LoginUserResponseDto {
                access_token,
                expires_in: 3600,
                token_type: "Bearer".to_string()
            };
        },
        Ok(None) => {
            println!("Usuário não encontrado.");
            return LoginUserResponseDto { access_token: "".to_string(), token_type: "".to_string(), expires_in: 0 };
        },
        Err(err) => {
            println!("Error ");
            eprint!("{}", err);
            return LoginUserResponseDto { access_token: "".to_string(), token_type: "".to_string(), expires_in: 0 };
        }
    }
}