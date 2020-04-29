use super::super::models::{Track, TransportTicket};
use super::connection::DBConnection;
use crate::schema::transportTicket::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct TransportTicketRepository<'a> {
  pub connection: &'a dyn DBConnection<Connection = PgConnection>,
}

pub trait TransportTicketRepositoryImpl {
  type Model;
  type Track;

  fn get_by_track(&self, track: &Self::Track) -> Result<Vec<Self::Model>, diesel::result::Error>;

  fn reset(&self, ticket: &TransportTicket) -> Result<(), diesel::result::Error>;
}

impl<'a> TransportTicketRepositoryImpl for TransportTicketRepository<'a> {
  type Model = TransportTicket;
  type Track = Track;

  fn get_by_track(&self, track: &Self::Track) -> Result<Vec<Self::Model>, diesel::result::Error> {
    TransportTicket::belonging_to(track).load::<TransportTicket>(&self.connection.get_connection())
  }

  fn reset(&self, ticket: &TransportTicket) -> Result<(), diesel::result::Error> {
    diesel::update(ticket)
      .set(trackId.eq::<Option<i32>>(None))
      .execute(&self.connection.get_connection())?;
    Ok(())
  }
}
