use async_trait::async_trait;

use crate::{
  dtos::list_income_user::{
    IncomeNormalized, ListIncomeByUserParamsDto, ListIncomeByUserResponseDto,
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
      Ok(incomes) => {
        let incomes_dto: Vec<IncomeNormalized> = incomes
          .into_iter()
          .map(|income| IncomeNormalized {
            id: income._id.to_hex(),
            group_id: income.group_id.map(|id| id.to_hex()),
            user_ids: income
              .user_ids
              .into_iter()
              .map(|id| id.to_hex())
              .collect(),
            title: income.title,
            date: income.date.to_rfc3339(),
            category: income.category,
            tag: income.tag,
            recurrent: income.recurrent,
            amount_in_cents: income.amount_in_cents,
            exclude_dates: income
              .exclude_dates
              .into_iter()
              .map(|d| d.to_rfc3339())
              .collect(),
            end_date: income.end_date.map(|d| d.to_rfc3339()),
            created_at: income.created_at.to_rfc3339(),
            updated_at: income.updated_at.to_rfc3339(),
            deleted_at: income.deleted_at.map(|d| d.to_rfc3339()),
          })
          .collect();

        Ok(ListIncomeByUserResponseDto {
          incomes: incomes_dto,
        })
      }
      Err(error) => {
        println!("{:?}", error);
        Err(InternalServerError::new("Internal server error").service_error)
      }
    }
  }
}
