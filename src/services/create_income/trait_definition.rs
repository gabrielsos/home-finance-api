use async_trait::async_trait;

use crate::{
  dtos::{
    create_income_dto::CreateIncomeParamsDto,
    create_income_dto::CreateIncomeResponseDto,
  },
  errors::service_error::ServiceError,
};

#[async_trait]
pub trait CreateIncomeService {
  async fn execute<'a>(
    &'a self,
    params: &'a CreateIncomeParamsDto,
  ) -> Result<CreateIncomeResponseDto, ServiceError>;
}
