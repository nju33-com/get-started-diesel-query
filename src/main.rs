use diesel::pg::PgConnection;
use diesel::prelude::*;
use get_started_diesel_query::{schema, User};

const DATABASE_URL: &'static str = "postgres://postgres:@localhost:5432";

fn main() {
  let connection =
    PgConnection::establish(DATABASE_URL).expect(&format!("Error connecting to {}", DATABASE_URL));

  let users = schema::users::dsl::users
    .load::<User>(&connection)
    .expect("Error loading users");

  println!("{:#?}", users);
}
