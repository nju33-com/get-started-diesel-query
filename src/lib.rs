#[macro_use]
extern crate diesel;

pub mod schema;

#[derive(Queryable, Debug)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub nickname: Option<String>,
  pub age: i32,
  pub active: bool,
  pub created_at: chrono::NaiveDateTime,
}
