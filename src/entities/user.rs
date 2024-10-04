use mongodb::bson::oid::ObjectId;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
  pub _id: ObjectId,
  pub name: String,
  pub email: String,
  pub password: String,
}