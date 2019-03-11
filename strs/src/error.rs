use crate::common::*;

#[derive(Debug)]
pub enum Error {
  Io(io::Error),
}

impl From<io::Error> for Error {
  fn from(io_error: io::Error) -> Error {
    Error::Io(io_error)
  }
}
