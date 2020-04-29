// use std::error;
// use std::fmt;
use thiserror::Error;

// #[derive(Debug, Clone)]
// pub struct RepositoryError(String);

// impl RepositoryError {
//   pub fn new(msg: &str) -> RepositoryError {
//     RepositoryError(msg.into())
//   }
// }

// impl fmt::Display for RepositoryError {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "{}", self.0)
//   }
// }

// impl error::Error for RepositoryError {
//   fn source(&self) -> Option<&(dyn error::Error + 'static)> {
//     None
//   }
// }

#[derive(Error, Debug)]
pub enum GatewayDBError {
  #[error(transparent)]
  DieselResultError(#[from] diesel::result::Error),
  #[error("Invalid connection string: {0}")]
  InvalidConnectionString(String),
}
