use actix_web::{post, web, HttpResponse, Responder};

use crate::{dtos::{create_user_dto::CreateUserParamsDto, login_user_dto::LoginUserParamsDto}, services::{create_user, login_user}};

#[post("/user")]
async fn register_user(data: web::Json<CreateUserParamsDto>) -> impl Responder {
  let user = create_user::execute(
    &CreateUserParamsDto{
      name: data.name.to_string(),
      email: data.email.to_string(),
      password: data.password.to_string()
    }
  ).await;

  HttpResponse::Ok().json(user)
}

#[post("/user/login")]
async fn login_user_controller(data: web::Json<LoginUserParamsDto>) -> impl Responder {
  let user = login_user::execute(
    &LoginUserParamsDto{
      email: data.email.to_string(),
      password: data.password.to_string()
    }
  ).await;

  HttpResponse::Ok().json(user)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user);
    cfg.service(login_user_controller);
}
