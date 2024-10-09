use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use crate::{
  dtos::{
    create_income_dto::{
      CreateIncomeControllerParamsDto, CreateIncomeParamsDto,
    },
    list_income_user::{
      ListIncomeByUserParamsDto, ListIncomeByUserQueryParams,
    },
  },
  middlewares::auth::is_valid_jwt,
  services::{
    create_income::{
      implementation::CreateIncomeServiceImpl,
      trait_definition::CreateIncomeService,
    },
    list_income_user_id::{
      implementation::ListIncomeByUserIdImpl,
      trait_definition::ListIncomeByUserId,
    },
  },
};

#[post("/user/{id}/income")]
async fn register_income_controller(
  data: web::Json<CreateIncomeControllerParamsDto>,
  req: HttpRequest,
  user_id: web::Path<String>,
) -> impl Responder {
  if !is_valid_jwt(&req.headers()).await {
    return HttpResponse::Unauthorized().finish();
  }

  match CreateIncomeServiceImpl
    .execute(&CreateIncomeParamsDto {
      user_ids: data.user_ids.clone().unwrap_or(Vec::new()),
      owner_user_id: user_id.to_string(),
      amount_in_cents: data.amount_in_cents,
      category: data.category.to_string(),
      date: data.date.to_string(),
      title: data.title.to_string(),
      group_id: data.group_id.clone(),
      recurrent: data.recurrent,
      tag: data.tag.clone(),
      end_date: data.end_date,
    })
    .await
  {
    Ok(group) => return HttpResponse::Ok().json(group),
    Err(err) => return HttpResponse::build(err.status_code).json(err.json),
  };
}

#[get("/user/{id}/income")]
async fn list_incomes_controller(
  req: HttpRequest,
  user_id: web::Path<String>,
  query: web::Query<ListIncomeByUserQueryParams>,
) -> impl Responder {
  if !is_valid_jwt(&req.headers()).await {
    return HttpResponse::Unauthorized().finish();
  }

  match ListIncomeByUserIdImpl
    .execute(&ListIncomeByUserParamsDto {
      date: query.date,
      user_id: user_id.clone(),
    })
    .await
  {
    Ok(incomes) => return HttpResponse::Ok().json(incomes),
    Err(err) => return HttpResponse::build(err.status_code).json(err.json),
  };
}

pub fn init(cfg: &mut web::ServiceConfig) {
  cfg.service(register_income_controller);
  cfg.service(list_incomes_controller);
}
