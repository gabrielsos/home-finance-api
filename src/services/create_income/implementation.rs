use async_trait::async_trait;
use chrono::Utc;
use futures::executor::block_on;
use mongodb::error::Error as MongoError;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
  dtos::create_income_dto::{CreateIncomeParamsDto, CreateIncomeResponseDto},
  entities::user::User,
  errors::{
    bad_request_error::BadRequestError,
    internal_server_error::InternalServerError, service_error::ServiceError,
  },
  repositories::{
    income_repository::IncomeRepository, user_repository::UserRepository,
  },
};

use super::trait_definition::CreateIncomeService;

pub struct CreateIncomeServiceImpl;

#[async_trait]
impl CreateIncomeService for CreateIncomeServiceImpl {
  async fn execute<'a>(
    &'a self,
    data: &'a CreateIncomeParamsDto,
  ) -> Result<CreateIncomeResponseDto, ServiceError> {
    let user_repository = UserRepository::new().await;

    let users: Vec<&String> = std::iter::once(&data.owner_user_id)
      .chain(data.user_ids.iter())
      .collect();

    let results: Vec<Result<Option<User>, MongoError>> = users
      .par_iter()
      .map(|user_id| block_on(user_repository.find_user_by_id(user_id)))
      .collect();

    for result in results {
      match result {
        Ok(Some(_)) => {}
        Ok(None) => {
          return Err(BadRequestError::new("Invalid user").service_error);
        }
        Err(_) => {
          return Err(BadRequestError::new("Invalid user").service_error);
        }
      }
    }

    let income_repository = IncomeRepository::new().await;

    match income_repository.create(data).await {
      Ok(income) => {
        return Ok(CreateIncomeResponseDto {
          id: income.inserted_id.to_string(),
          owner_user_id: data.owner_user_id.clone(),
          category: data.category.to_string(),
          date: data.date.clone(),
          title: data.title.to_string(),
          amount_in_cents: data.amount_in_cents,
          created_at: Utc::now(),
          updated_at: Utc::now(),
          group_id: data.group_id.clone(),
          recurrent: data.recurrent,
          end_date: data.end_date,
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
