use super::super::models::Track;
use super::super::schema::track::dsl::*;
use super::connection::DBConnection;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct TrackRepository<'a> {
  pub connection: &'a dyn DBConnection<Connection = PgConnection>,
}

pub trait TrackRepositoryImpl {
  type Model;

  fn get_by_number(&self, track_number: &i32) -> Result<Self::Model, diesel::result::Error>;
}

impl<'a> TrackRepositoryImpl for TrackRepository<'a> {
  type Model = Track;

  fn get_by_number(&self, track_number: &i32) -> Result<Self::Model, diesel::result::Error> {
    track
      .filter(number.eq(track_number))
      .first::<Self::Model>(&self.connection.get_connection())
  }
}
