use mongodb::bson::{doc, DateTime, Document};
use mongodb::results::InsertOneResult;
use mongodb::Collection;
use mongodb::error::Result;

use crate::database;
use crate::dtos::create_group_dto::CreateGroupParamsDto;
pub struct GroupRepository {
    collection: Collection<Document>,
}

impl GroupRepository {
  pub async fn new() -> Self {
    let db = database::get_db().await;
    let collection = db.collection("groups");
    GroupRepository { collection }
  }

  pub async fn create(&self, user_data: &CreateGroupParamsDto) -> Result<InsertOneResult> {
    let bson_now = DateTime::now();

    let group_data_doc = doc! {
        "name": &user_data.name,
        "description": &user_data.description,
        "user_ids": &user_data.user_ids,
        "created_at": bson_now,
        "updated_at": bson_now
    };

    let insert_result = self.collection.insert_one(group_data_doc).await?;

    Ok(insert_result)
  }
}
