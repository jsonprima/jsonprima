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

  /// Illegal character "n" after structural token. Expected comma or colon.
  E108,

  /// Illegal number after structural token. Expected comma or colon.
  E109,

  /// Illegal non ASCII decimal digit character in number.
  E110,

  /// Numbers cannot contain leading zeros.
  E111,

  /// Could not parse out of range JSON number.
  E112,

  /// Could not parse JSON number.
  E113,
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
      ErrorType::E108 => {
        "Illegal character \"n\" after structural token. Expected comma or colon."
      }
      ErrorType::E109 => {
        "Illegal number after structural token. Expected comma or colon."
      }
      ErrorType::E110 => "Illegal non ASCII decimal digit character in number.",
      ErrorType::E111 => "Numbers cannot contain leading zeros.",
      ErrorType::E112 => "Could not parse out of range JSON number.",
      ErrorType::E113 => "Could not parse JSON number.",
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
      ErrorType::E108 => "E108",
      ErrorType::E109 => "E109",
      ErrorType::E110 => "E110",
      ErrorType::E111 => "E111",
      ErrorType::E112 => "E112",
      ErrorType::E113 => "E113",
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
