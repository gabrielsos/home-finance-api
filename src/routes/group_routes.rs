use actix_web::{post, web, HttpRequest, HttpResponse, Responder};

use crate::{dtos::create_group_dto::CreateGroupParamsDto, middlewares::auth::is_valid_jwt, services::create_group};

#[post("/group")]
async fn register_group_controller(data: web::Json<CreateGroupParamsDto>, req: HttpRequest) -> impl Responder {
  if !is_valid_jwt(&req.headers()).await {
    return HttpResponse::Unauthorized().finish()
  }

  let group = create_group::execute(
    &CreateGroupParamsDto{
      name: data.name.to_string(),
      description: data.description.as_ref().map(|desc| desc.clone()),
      user_ids: data.user_ids.to_vec()
    }
  ).await;

  HttpResponse::Ok().json(group)
}

pub fn init(cfg: &mut web::ServiceConfig) {
  cfg.service(register_group_controller);
}