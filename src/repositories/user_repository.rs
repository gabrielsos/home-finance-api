use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, from_document, Document, DateTime};
use mongodb::results::InsertOneResult;
use mongodb::Collection;
use mongodb::error::Result;

use crate::database;
use crate::dtos::create_user_dto::CreateUserParamsDto;
use crate::entities::user::User;

pub struct UserRepository {
    collection: Collection<Document>,
}

impl UserRepository {
  pub async fn new() -> Self {
    let db = database::get_db().await;
    let collection = db.collection("users");
    UserRepository { collection }
  }

  pub async fn create_user(&self, user_data: &CreateUserParamsDto) -> Result<InsertOneResult> {
    let bson_now = DateTime::now();

    let user_data_doc = doc! {
        "name": &user_data.name,
        "email": &user_data.email,
        "password": &user_data.password,
        "created_at": bson_now,
        "updated_at": bson_now
    };

    let insert_result = self.collection.insert_one(user_data_doc).await?;

    Ok(insert_result)
  }

  pub async fn find_user_by_email(&self, email: &String) -> Result<Option<User>> {
    let user_data_doc = doc! {
        "email": email.to_string(),
    };

    let user_doc = self.collection.find_one(user_data_doc).await?;

    match user_doc {
      Some(doc) => {
          let user: User = from_document(doc)?;
          Ok(Some(user))
      },
      None => Ok(None),
    }
  }

  pub async fn find_user_by_id(&self, user_id: &String) -> Result<Option<User>> {
    let user_data_doc = doc! {
        "_id": ObjectId::from_str(&user_id).unwrap(),
    };

    let user_doc = self.collection.find_one(user_data_doc).await?;

    match user_doc {
      Some(doc) => {
          let user: User = from_document(doc)?;
          Ok(Some(user))
      },
      None => Ok(None),
    }
  }
}
