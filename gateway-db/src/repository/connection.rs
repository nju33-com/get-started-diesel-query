use diesel::pg::PgConnection;
use diesel::Connection;

pub trait DBConnection {
  type Connection;

  fn get_connection(&self) -> Self::Connection;
}

pub struct EcogConnection {
  pub connection_string: String,
}

impl EcogConnection {
  pub fn new<T: Into<String>>(connection_string: T) -> Self {
    EcogConnection {
      connection_string: connection_string.into(),
    }
  }
}

impl DBConnection for EcogConnection {
  type Connection = PgConnection;

  fn get_connection(&self) -> Self::Connection {
    PgConnection::establish(&self.connection_string)
      .expect(&format!("Error connecting to {}", &self.connection_string))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct MockConnection {
    pub connection: (),
  }

  impl DBConnection for MockConnection {
    type Connection = ();

    fn get_connection(&self) -> Self::Connection {
      ()
    }
  }

  #[test]
  fn establish_success() {
    assert_eq!(MockConnection { connection: () }.get_connection(), ());
  }
}
