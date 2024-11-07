use chrono::{Datelike, NaiveDate, Utc};
use futures::TryStreamExt;
use mongodb::bson::{doc, from_document, DateTime, Document};
use mongodb::error::Result;
use mongodb::results::InsertOneResult;
use mongodb::Collection;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::database;
use crate::dtos::create_income_dto::CreateIncomeParamsDto;
use crate::dtos::list_income_user::ListIncomeByUserParamsDto;
use crate::entities::income::Income;
use crate::utils::crypto::{decrypt_data, encrypt_data};
use crate::utils::timestamps::DocumentWithTimestamps;
pub struct IncomeRepository {
  collection: Collection<Document>,
}

impl IncomeRepository {
  pub async fn new() -> Self {
    let db = database::get_db().await;
    let collection = db.collection("income");
    IncomeRepository { collection }
  }

  pub async fn create(
    &self,
    user_data: &CreateIncomeParamsDto,
  ) -> Result<InsertOneResult> {
    let group_data_doc = doc! {
        "title": encrypt_data(&user_data.title),
        "user_ids": &user_data.user_ids,
        "group_id": &user_data.group_id,
        "date": &user_data.date,
        "category": encrypt_data(&user_data.category),
        "tag": user_data.tag.as_ref().map(|tag| encrypt_data(tag)),
        "recurrent": &user_data.recurrent,
        "amount_in_cents": &user_data.amount_in_cents,
        "end_date": user_data.end_date.map(|date| DateTime::from_chrono(date)),
    }
    .with_timestamps();

    let insert_result = self.collection.insert_one(group_data_doc).await?;

    Ok(insert_result)
  }

  pub async fn find_all_by_user(
    &self,
    user_data: &ListIncomeByUserParamsDto,
  ) -> Result<Vec<Income>> {
    let mut filter = doc! {
      "owner_user_id": &user_data.user_id
    };

    if let Some(date) = user_data.date {
      let year = date.year();
      let month = date.month();

      let start_date = DateTime::from_chrono(
        NaiveDate::from_ymd_opt(year, month, 1)
          .unwrap()
          .and_hms_opt(0, 0, 0)
          .unwrap()
          .and_local_timezone(Utc)
          .unwrap(),
      );

      let last_day = NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap()
        .pred_opt()
        .unwrap();

      let end_date = DateTime::from_chrono(
        last_day
          .and_hms_opt(23, 59, 59)
          .unwrap()
          .and_local_timezone(Utc)
          .unwrap(),
      );

      filter.insert(
        "date",
        doc! {
            "$gte": start_date,
            "$lt": end_date
        },
      );
    }

    let mut cursor = self.collection.find(filter).await?;

    let mut incomes: Vec<Income> = Vec::new();

    while let Ok(Some(doc)) = cursor.try_next().await {
      match from_document::<Income>(doc) {
        Ok(mut income) => {
          let mut data = vec![&income.title, &income.category];

          if let Some(tag) = &income.tag {
            data.push(tag);
          }

          let decrypted: Vec<String> =
            data.into_par_iter().map(|d| decrypt_data(d)).collect();

          income.title = decrypted[0].clone();
          income.category = decrypted[1].clone();

          if let Some(_) = income.tag {
            income.tag = Some(decrypted[2].clone());
          }

          incomes.push(income)
        }
        Err(e) => return Err(e.into()),
      }
    }

    Ok(incomes)
  }
}
