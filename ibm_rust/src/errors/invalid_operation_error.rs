use std::fmt;

#[derive(Debug)]
pub struct InvalidOperationError;

impl fmt::Display for InvalidOperationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "An invalid operation was passed when parsing the input JSON data.")
  }
}

