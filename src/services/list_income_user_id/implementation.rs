use async_trait::async_trait;

use crate::{
  dtos::list_income_user::{
    ListIncomeByUserParamsDto, ListIncomeByUserResponseDto,
  },
  errors::{
    internal_server_error::InternalServerError, service_error::ServiceError,
  },
  repositories::income_repository::IncomeRepository,
};

use super::trait_definition::ListIncomeByUserId;

pub struct ListIncomeByUserIdImpl;

#[async_trait]
impl ListIncomeByUserId for ListIncomeByUserIdImpl {
  async fn execute<'a>(
    &'a self,
    data: &'a ListIncomeByUserParamsDto,
  ) -> Result<ListIncomeByUserResponseDto, ServiceError> {
    let income_repository: IncomeRepository = IncomeRepository::new().await;

    match income_repository.find_all_by_user(&data).await {
      Ok(incomes) => Ok(ListIncomeByUserResponseDto { incomes }),
      Err(error) => {
        println!("{:?}", error);
        Err(InternalServerError::new("Internal server error").service_error)
      }
    }
  }
}
