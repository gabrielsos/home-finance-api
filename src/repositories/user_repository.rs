use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, from_document, Document};
use mongodb::error::Result;
use mongodb::results::InsertOneResult;
use mongodb::Collection;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::database;
use crate::dtos::create_user_dto::CreateUserParamsDto;
use crate::entities::user::User;
use crate::utils::crypto::{decrypt_data, encrypt_data};
use crate::utils::timestamps::DocumentWithTimestamps;

pub struct UserRepository {
  collection: Collection<Document>,
}

impl UserRepository {
  pub async fn new() -> Self {
    let db = database::get_db().await;
    let collection = db.collection("users");
    UserRepository { collection }
  }

  pub async fn create_user(
    &self,
    user_data: &CreateUserParamsDto,
  ) -> Result<InsertOneResult> {
    let user_data_doc = doc! {
        "name": encrypt_data(&user_data.name),
        "email": encrypt_data(&user_data.email),
        "password": encrypt_data(&user_data.password),
    }
    .with_timestamps();

    let insert_result = self.collection.insert_one(user_data_doc).await?;

    Ok(insert_result)
  }

  pub async fn find_user_by_email(
    &self,
    email: &String,
  ) -> Result<Option<User>> {
    let user_data_doc = doc! {
        "email": encrypt_data(&email.to_string()),
    };

    let user_doc = self.collection.find_one(user_data_doc).await?;

    match user_doc {
      Some(doc) => {
        let mut user: User = from_document(doc)?;

        let data = vec![&user.name, &user.email, &user.password];
        let decrypted: Vec<String> =
          data.into_par_iter().map(|d| decrypt_data(d)).collect();

        user.name = decrypted[0].clone();
        user.email = decrypted[1].clone();
        user.password = decrypted[2].clone();

        Ok(Some(user))
      }
      None => Ok(None),
    }
  }

  pub async fn find_user_by_id(
    &self,
    user_id: &String,
  ) -> Result<Option<User>> {
    let id = match ObjectId::from_str(&user_id) {
      Ok(object_id) => object_id,
      Err(_) => return Ok(None),
    };

    let user_data_doc = doc! {
        "_id": id,
    };

    let user_doc = self.collection.find_one(user_data_doc).await?;

    match user_doc {
      Some(doc) => {
        let user: User = from_document(doc)?;
        Ok(Some(user))
      }
      None => Ok(None),
    }
  }
}
