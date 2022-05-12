use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

pub struct User {
  pub id: String,
  pub pseudo: Option<String>,
  pub email: String,
  pub password_hash: String,
}
