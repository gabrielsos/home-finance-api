use actix_web::{post, web, HttpResponse, Responder};

use crate::dtos::{
  create_user_dto::CreateUserParamsDto, login_user_dto::LoginUserParamsDto,
};
use crate::services::create_user::implementation::CreateUserServiceImpl;
use crate::services::create_user::trait_definition::CreateUserService;
use crate::services::login_user::implementation::LoginUserServiceImpl;
use crate::services::login_user::trait_definition::LoginUserService;

#[post("/user")]
async fn register_user(data: web::Json<CreateUserParamsDto>) -> impl Responder {
  match CreateUserServiceImpl
    .execute(&CreateUserParamsDto {
      name: data.name.to_string(),
      email: data.email.to_string(),
      password: data.password.to_string(),
    })
    .await
  {
    Ok(user) => HttpResponse::Ok().json(user),
    Err(err) => HttpResponse::build(err.status_code).json(err.json),
  }
}

#[post("/user/login")]
async fn login_user_controller(
  data: web::Json<LoginUserParamsDto>,
) -> impl Responder {
  match LoginUserServiceImpl
    .execute(&LoginUserParamsDto {
      email: data.email.to_string(),
      password: data.password.to_string(),
    })
    .await
  {
    Ok(login) => return HttpResponse::Ok().json(login),
    Err(err) => return HttpResponse::build(err.status_code).json(err.json),
  };
}

pub fn init(cfg: &mut web::ServiceConfig) {
  cfg.service(register_user);
  cfg.service(login_user_controller);
}
