//! Error types

// TODO: Add documentation examples to `description`, `code` and `new` functions.

/// A list of all error variants used by this module.
#[derive(Debug)]
pub enum ErrorType {
  /// Empty JSON document.
  E100,

  /// Illegal character "t" after structural token. Expected comma or colon.
  E103,

  /// No more graphemes to parse.
  E104,

  /// Invalid character in literal name.
  E105,

  /// Invalid literal.
  E106,

  /// Illegal character "f" after structural token. Expected comma or colon.
  E107,
}

/// Used to represent the various error variants in this module.
impl ErrorType {
  /// Return the error description.
  pub fn description(&self) -> &str {
    match self {
      ErrorType::E100 => "Empty JSON document.",
      ErrorType::E103 => {
        "Illegal character \"t\" after structural token. Expected comma or colon."
      }
      ErrorType::E104 => "No more graphemes to parse.",
      ErrorType::E105 => "Invalid character in literal name.",
      ErrorType::E106 => "Invalid literal.",
      ErrorType::E107 => {
        "Illegal character \"f\" after structural token. Expected comma or colon."
      }
    }
  }

  /// Return the error code.
  pub fn code(&self) -> &str {
    match self {
      ErrorType::E100 => "E100",
      ErrorType::E103 => "E103",
      ErrorType::E104 => "E104",
      ErrorType::E105 => "E105",
      ErrorType::E106 => "E106",
      ErrorType::E107 => "E107",
    }
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
