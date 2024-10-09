use mongodb::bson::{doc, Document};
use mongodb::error::Result;
use mongodb::results::InsertOneResult;
use mongodb::Collection;

use crate::database;
use crate::dtos::create_income_dto::CreateIncomeParamsDto;
use crate::utils::crypto::encrypt_data;
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
    }
    .with_timestamps();

    let insert_result = self.collection.insert_one(group_data_doc).await?;

    Ok(insert_result)
  }
}
