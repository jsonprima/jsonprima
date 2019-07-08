//! Error types

// TODO: Add documentation examples to `description`, `code` and `new` functions.

/// A list of all error variants used by this module.
#[derive(Debug)]
pub enum ErrorType {
  /// Empty JSON document.
  E100,

  /// Raw use of control characters in JSON string.
  E101,

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

  /// Illegal string after structural token. Expected comma or colon.
  E114,

  // Invalid escape character in JSON string.
  E116,

  // Invalid character found in Unicode escape sequence.
  E117,

  // Invalid Unicode grapheme in JSON string.
  E118,

  // Invalid Unicode escape sequence in second surrogate pair.
  E119,

  // Unexpected comma at the start of JSON document.
  E122,

  // Unexpected comma after structural token.
  E123,

  // Invalid use of comma after document root value.
  E124,

  // Illegal begin-array after JSON value.
  E125,

  // Illegal end-array. No begin-array match.
  E126,

  // Unterminated array.
  E127,

  // Unterminated object.
  E128,

  // Illegal end-array after comma.
  E129,

  // Illegal begin-object after JSON value.
  E130,

  // Illegal end-object. No begin-object match.
  E131,

  // Illegal end-object after comma.
  E132,

  // Illegal end-array after colon.
  E133,

  // Illegal end-object after colon.
  E134,

  // Invalid object member. Member value does not exist.
  E135,

  // Invalid use of colon.
  E136,

  // Invalid use of number as object name.
  E137,

  // Invalid use of true as object name.
  E138,

  // Invalid use of false as object name.
  E139,

  // Invalid use of null as object name.
  E140,

  // Invalid object member
  E141,

  // Invalid use of array as object name.
  E142,

  // Invalid use of object as object name.
  E143,
}

/// Used to represent the various error variants in this module.
impl ErrorType {
  /// Return the error description.
  pub fn description(&self) -> &str {
    match self {
      ErrorType::E100 => "Empty JSON document.",
      ErrorType::E101 => "Raw use of control characters in JSON string.",
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
      ErrorType::E114 => {
        "Illegal string after structural token. Expected comma or colon."
      }
      ErrorType::E116 => "Invalid escape character in JSON string.",
      ErrorType::E117 => "Invalid character found in Unicode escape sequence.",
      ErrorType::E118 => "Invalid Unicode grapheme in JSON string.",
      ErrorType::E119 => "Invalid Unicode escape sequence in second surrogate pair.",
      ErrorType::E122 => "Unexpected comma at the start of JSON document.",
      ErrorType::E123 => "Unexpected comma after structural token.",
      ErrorType::E124 => "Invalid use of comma after document root value.",
      ErrorType::E125 => "Illegal begin-array after JSON value.",
      ErrorType::E126 => "Illegal end-array. No begin-array match.",
      ErrorType::E127 => "Unterminated array.",
      ErrorType::E128 => "Unterminated object.",
      ErrorType::E129 => "Illegal end-array after comma.",
      ErrorType::E130 => "Illegal begin-object after JSON value.",
      ErrorType::E131 => "Illegal end-object. No begin-object match.",
      ErrorType::E132 => "Illegal end-object after comma.",
      ErrorType::E133 => "Illegal end-array after colon.",
      ErrorType::E134 => "Illegal end-object after colon.",
      ErrorType::E135 => "Invalid object member. Member value does not exist.",
      ErrorType::E136 => "Invalid use of colon.",
      ErrorType::E137 => "Invalid use of number as object name.",
      ErrorType::E138 => "Invalid use of true as object name.",
      ErrorType::E139 => "Invalid use of false as object name.",
      ErrorType::E140 => "Invalid use of null as object name.",
      ErrorType::E141 => "Invalid object member.",
      ErrorType::E142 => "Invalid use of array as object name.",
      ErrorType::E143 => "Invalid use of object as object name.",
    }
  }

  /// Return the error code.
  pub fn code(&self) -> &str {
    match self {
      ErrorType::E100 => "E100",
      ErrorType::E101 => "E101",
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
      ErrorType::E114 => "E114",
      ErrorType::E116 => "E116",
      ErrorType::E117 => "E117",
      ErrorType::E118 => "E118",
      ErrorType::E119 => "E119",
      ErrorType::E122 => "E122",
      ErrorType::E123 => "E123",
      ErrorType::E124 => "E124",
      ErrorType::E125 => "E125",
      ErrorType::E126 => "E126",
      ErrorType::E127 => "E127",
      ErrorType::E128 => "E128",
      ErrorType::E129 => "E129",
      ErrorType::E130 => "E130",
      ErrorType::E131 => "E131",
      ErrorType::E132 => "E132",
      ErrorType::E133 => "E133",
      ErrorType::E134 => "E134",
      ErrorType::E135 => "E135",
      ErrorType::E136 => "E136",
      ErrorType::E137 => "E137",
      ErrorType::E138 => "E138",
      ErrorType::E139 => "E139",
      ErrorType::E140 => "E140",
      ErrorType::E141 => "E141",
      ErrorType::E142 => "E142",
      ErrorType::E143 => "E143",
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
