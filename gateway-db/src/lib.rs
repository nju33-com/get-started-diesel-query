#[macro_use]
extern crate diesel;

use crate::models::{Track, TransportTicket};
use crate::repository::connection::DBConnection;
use crate::repository::track::TrackRepositoryImpl;
use crate::repository::transport_ticket::TransportTicketRepositoryImpl;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod error;
pub mod models;
pub mod repository;
pub mod schema;

/// CONNECTION_STRING の形式が正しいか確認
fn is_correct_connection_string(connection_string: &str) -> bool {
  connection_string.starts_with("postgres://")
}

struct ResetTicketsTransaction<'a> {
  pub connection: &'a dyn DBConnection<Connection = PgConnection>,
  pub track_repository: &'a dyn TrackRepositoryImpl<Model = Track>,
  pub transport_ticktet_repository:
    &'a dyn TransportTicketRepositoryImpl<Model = TransportTicket, Track = Track>,
}

trait ResetTicketsTransactionImpl {
  fn execute(&self, track_number: &i32) -> Result<(), diesel::result::Error>;
}

impl<'a> ResetTicketsTransactionImpl for ResetTicketsTransaction<'a> {
  fn execute(&self, track_number: &i32) -> Result<(), diesel::result::Error> {
    let connection = self.connection.get_connection();

    connection.transaction::<(), diesel::result::Error, _>(|| {
      let track_result = self.track_repository.get_by_number(track_number)?;
      let tickets = self
        .transport_ticktet_repository
        .get_by_track(&track_result)?;

      for ticket in tickets.into_iter() {
        self.transport_ticktet_repository.reset(&ticket)?;
      }

      Ok(())
    })
  }
}

/// track_number な track_id を持つ transportTicket の track_id
/// を null にリセット
pub fn reset_tickets_by_track_number(
  connection_string: &str,
  track_number: &i32,
) -> Result<(), error::GatewayDBError> {
  if !is_correct_connection_string(connection_string) {
    return Err(error::GatewayDBError::InvalidConnectionString(
      connection_string.into(),
    ));
  }

  let connection = repository::connection::EcogConnection::new(connection_string);

  let track_repository = repository::track::TrackRepository {
    connection: &connection,
  };
  let transport_ticktet_repository = repository::transport_ticket::TransportTicketRepository {
    connection: &connection,
  };

  let reset_tickets_transaction = ResetTicketsTransaction {
    connection: &connection,
    track_repository: &track_repository,
    transport_ticktet_repository: &transport_ticktet_repository,
  };
  reset_tickets_transaction.execute(track_number)?;

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::models::{Track, TransportTicket};
  use crate::repository::connection::DBConnection;
  use crate::repository::track::TrackRepositoryImpl;
  use crate::repository::transport_ticket::TransportTicketRepositoryImpl;
  use crate::ResetTicketsTransactionImpl;

  mod is_correct_connection_string {
    use super::super::*;

    #[test]
    fn return_true() {
      assert!(is_correct_connection_string(
        "postgres://postgres:@localhost:5432"
      ));
    }

    #[test]
    fn return_false() {
      assert!(!is_correct_connection_string("http://localhost:5432"));
    }
  }

  struct MockConnection;

  impl MockConnection {
    #[allow(dead_code)]
    pub fn transaction(f: &dyn Fn()) -> Result<(), diesel::result::Error> {
      Ok(f())
    }
  }

  struct AConnection {
    #[allow(dead_code)]
    connection_string: String,
  }

  impl DBConnection for AConnection {
    type Connection = MockConnection;

    fn get_connection(&self) -> Self::Connection {
      MockConnection
    }
  }

  struct MockTrackRepository<'a> {
    pub connection: &'a dyn DBConnection<Connection = MockConnection>,
  }

  impl<'a> TrackRepositoryImpl for MockTrackRepository<'a> {
    type Model = Track;

    fn get_by_number(&self, track_number: &i32) -> Result<Track, diesel::result::Error> {
      Ok(Track {
        id: 10,
        number: track_number.clone(),
      })
    }
  }

  struct MockTransportTicketRepository<'a> {
    pub connection: &'a dyn DBConnection<Connection = MockConnection>,
  }

  impl<'a> TransportTicketRepositoryImpl for MockTransportTicketRepository<'a> {
    type Model = TransportTicket;
    type Track = Track;

    fn get_by_track(
      &self,
      _track: &Self::Track,
    ) -> Result<Vec<TransportTicket>, diesel::result::Error> {
      Ok(vec![TransportTicket {
        id: 100,
        track_id: Some(15),
      }])
    }

    fn reset(&self, _ticket: &Self::Model) -> Result<(), diesel::result::Error> {
      Ok(())
    }
  }

  struct MockResetTicketsTransaction<'a> {
    pub connection: &'a dyn DBConnection<Connection = MockConnection>,
    pub track_repository: &'a dyn TrackRepositoryImpl<Model = Track>,
    pub transport_ticktet_repository:
      &'a dyn TransportTicketRepositoryImpl<Model = TransportTicket, Track = Track>,
  }

  impl<'a> ResetTicketsTransactionImpl for MockResetTicketsTransaction<'a> {
    fn execute(&self, track_number: &i32) -> Result<(), diesel::result::Error> {
      let _connection = self.connection.get_connection();

      let track = self.track_repository.get_by_number(track_number)?;
      let _tickets = self.transport_ticktet_repository.get_by_track(&track)?;

      let track_result = self.track_repository.get_by_number(track_number)?;
      let tickets = self
        .transport_ticktet_repository
        .get_by_track(&track_result)?;

      for ticket in tickets.into_iter() {
        self.transport_ticktet_repository.reset(&ticket)?;
      }

      Ok(())
    }
  }

  #[test]
  fn can_reset() -> anyhow::Result<()> {
    use super::*;

    let connection = AConnection {
      connection_string: "dummy".into(),
    };

    let track_repository = MockTrackRepository {
      connection: &connection,
    };
    let transport_ticktet_repository = MockTransportTicketRepository {
      connection: &connection,
    };

    let reset_tickets_transaction = MockResetTicketsTransaction {
      connection: &connection,
      track_repository: &track_repository,
      transport_ticktet_repository: &transport_ticktet_repository,
    };
    reset_tickets_transaction.execute(&9000)?;

    Ok(())
  }
}
