use async_trait::async_trait;

use crate::{
  dtos::list_income_user::{
    ListIncomeByUserParamsDto, ListIncomeByUserResponseDto,
  },
  errors::service_error::ServiceError,
};

#[async_trait]
pub trait ListIncomeByUserId {
  async fn execute<'a>(
    &'a self,
    data: &'a ListIncomeByUserParamsDto,
  ) -> Result<ListIncomeByUserResponseDto, ServiceError>;
}
