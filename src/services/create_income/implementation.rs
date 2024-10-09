use async_trait::async_trait;
use chrono::Local;

use crate::{
  dtos::create_income_dto::{CreateIncomeParamsDto, CreateIncomeResponseDto},
  errors::{
    internal_server_error::InternalServerError, service_error::ServiceError,
  },
  repositories::income_repository::IncomeRepository,
};

use super::trait_definition::CreateIncomeService;

pub struct CreateIncomeServiceImpl;

#[async_trait]
impl CreateIncomeService for CreateIncomeServiceImpl {
  async fn execute<'a>(
    &'a self,
    data: &'a CreateIncomeParamsDto,
  ) -> Result<CreateIncomeResponseDto, ServiceError> {
    let income_repository = IncomeRepository::new().await;

    match income_repository.create(data).await {
      Ok(income) => {
        return Ok(CreateIncomeResponseDto {
          id: income.inserted_id.to_string(),
          category: data.category.to_string(),
          date: data.date.clone(),
          title: data.title.to_string(),
          amount_in_cents: data.amount_in_cents,
          created_at: Local::now(),
          updated_at: Local::now(),
          group_id: data.group_id.clone(),
          recurrent: data.recurrent,
          tag: data.tag.clone(),
          user_ids: data.user_ids.clone(),
        })
      }
      Err(_) => {
        return Err(
          InternalServerError::new("Internal server error").service_error,
        )
      }
    };
  }
}
