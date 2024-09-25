use actix_web::{post, web, HttpResponse, Responder};

use crate::{dtos::create_user_dto::CreateUserDto, services::create_user};

#[post("/user")]
async fn register_user(data: web::Json<CreateUserDto>) -> impl Responder {
  let user = create_user::execute(
    CreateUserDto{
      name: data.name.to_string(),
      email: data.email.to_string()
    }
  ).await;

  HttpResponse::Ok().json(user)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user);
}
