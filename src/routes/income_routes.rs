use actix_web::{post, web, HttpRequest, HttpResponse, Responder};

use crate::{
  dtos::create_income_dto::CreateIncomeParamsDto,
  middlewares::auth::is_valid_jwt,
  services::create_income::{
    implementation::CreateIncomeServiceImpl,
    trait_definition::CreateIncomeService,
  },
};

#[post("/user/{id}/income")]
async fn register_income_controller(
  data: web::Json<CreateIncomeParamsDto>,
  req: HttpRequest,
) -> impl Responder {
  if !is_valid_jwt(&req.headers()).await {
    return HttpResponse::Unauthorized().finish();
  }

  match CreateIncomeServiceImpl
    .execute(&CreateIncomeParamsDto {
      user_ids: data.user_ids.to_vec(),
      amount_in_cents: data.amount_in_cents,
      category: data.category.to_string(),
      date: data.date.to_string(),
      title: data.title.to_string(),
      group_id: data.group_id.clone(),
      recurrent: data.recurrent,
      tag: data.tag.clone(),
    })
    .await
  {
    Ok(group) => return HttpResponse::Ok().json(group),
    Err(err) => return HttpResponse::build(err.status_code).json(err.json),
  };
}

pub fn init(cfg: &mut web::ServiceConfig) {
  cfg.service(register_income_controller);
}
