//! Error types

// TODO: Add documentation examples to `description`, `code` and `new` functions.

/// A list of all error variants used by this module.
#[derive(Debug)]
pub enum ErrorType {}

/// Used to represent the various error variants in this module.
impl ErrorType {
  /// Return the error description.
  pub fn description(&self) -> &str {
    unimplemented!();
  }

  /// Return the error code.
  pub fn code(&self) -> &str {
    unimplemented!();
  }
}

/// Error type of validation functions.
#[derive(Debug)]
pub struct Error {
  /// The ErrorType variant that describes the current error.
  pub err: ErrorType,

  /// The zero-based index before which the error ends.
  /// The character at this index will not be included.
  pub index_end: usize,

  /// The zero-based index at which the error begins.
  pub index_start: usize,
}

impl Error {
  /// Construct a new error in the specified range.
  pub fn new(err: ErrorType, index_start: usize, index_end: usize) -> Error {
    Error {
      err,
      index_start,
      index_end,
    }
  }
}
