//! `jsonprima` is a RFC 8259 compliant JSON validator in Rust.
//! This module comes as a Cargo library or CLI version.
//!
//! ## CLI Usage
//! You can grab the [latest release](https://github.com/jsonprima/jsonprima/releases/latest) of the binary on GitHub.
//!
//! Pass the JSON document to validate as argument using  the `-i` option.
//!
//! ```bash
//! $ jsonprima -i "[true, false]"
//! []
//! ```
//!
//! The returned value is an JSON array with the returned errors as described bellow:
//!
//! ```ts
//! interface Error {
//!   code: string,
//!   description: string,
//!   index_start: number,
//!   index_end: number
//! }
//! ```
//!
//! In the above example the JSON document is valid, so the array does not contain any errors.
//!
//! Here is an example of a wrong JSON document:
//!
//! ```bash
//! $ jsonprima -i "trua"
//! "[{\"code\": \"E106\", \"description\": \"Invalid literal.\", \"index_end\": 2, \"index_start\": 1}]"
//! ```
//!
//! **Note:** This is a non-tolerant parser, expect that there will be at most one error in the returned array.

mod error;
mod json;
mod scanner;

use json::general_tokens::*;
use json::{StackTokens, JSON};
use scanner::Scanner;

mod validate_true;
use validate_true::validate_true;

mod validate_false;
use validate_false::validate_false;

mod validate_null;
use validate_null::validate_null;

mod validate_number;
use validate_number::validate_number;

mod validate_string;
use validate_string::validate_string;

mod validate_begin_array;
use validate_begin_array::validate_begin_array;

mod validate_end_array;
use validate_end_array::validate_end_array;

mod validate_value_separator;
use validate_value_separator::validate_value_separator;

mod validate_begin_object;
use validate_begin_object::validate_begin_object;

mod validate_end_object;
use validate_end_object::validate_end_object;

mod validate_name_separator;
use validate_name_separator::validate_name_separator;

// Public exports
pub use error::{Error, ErrorType};

/// Validate a JSON document based on RFC 8259 latest standard.
///
/// This function returns when encounter an error, i.e. it is not error tolerant.
///
/// **Notes**
/// - There is no limit to the size of the provided text to validate.
/// - There is no limit to the nesting of the JSON document.
/// - Ignores BOM, if it is present in the beginning of a JSON document,
///   without failing.
///
/// **Examples**
///
/// ```rust
/// // A valid JSON document.
/// let text: &str = "[true, false]";
/// let errors = jsonprima::validate(&text);
/// assert!(errors.is_empty()); // No errors, valid JSON document.
/// ```
///
/// ```rust
/// // Invalid `true` root value in JSON document.
/// let text: &str = "trua";
/// let errors = jsonprima::validate(&text);
///
/// assert_eq!(errors.get(0).unwrap().err.code(), "E105");
/// assert_eq!(errors.get(0).unwrap().index_start, 0);
/// assert_eq!(errors.get(0).unwrap().index_end, 4);
/// ```
pub fn validate(code: &str) -> Vec<Error> {
  // Create a scanner to operate in the text representation of the JSON document.
  let mut scanner = Scanner::new(code);

  // Create a `JSON` instance to save information regarding to validation process.
  let mut json_document = JSON::new();

  // Ignore BOM if present.
  if scanner
    .peek()
    .filter(|(_, first_character)| first_character == &'\u{feff}')
    .is_some()
  {
    scanner.next();
  }

  // Iterate over all characters of the JSON document, matching the token type
  // of each character (begin-array, horizontal-tab, etc).
  // We check the first character of a JSON value to determine
  // what value to validate, i.e. string, number, literal name, etc.
  //
  // For instance if the current character is "t", we know that we have to
  // validate for "true" JSON literal name. We pass the appropriate structures
  //  (`scanner`, `json_document`) down to `validate_true` function which is responsible
  // for the validation process. If the operation is successful, we return from the
  // `validate_true` function and the next `scanner` entry is the first character
  // after "true". In case of invalid JSON value, we return the error vector.
  while let Some((current_index, current_character)) = scanner.next() {
    match current_character {
      // Insignificant whitespace is always valid in a JSON document.
      SPACE | NEW_LINE | CARRIAGE_RETURN | HORIZONTAL_TAB => continue,

      // Character `t` is the first character of the `true` literal name.
      't' => {
        if validate_true(&mut json_document, &mut scanner).is_err() {
          return json_document.errors;
        }
      }

      // Character `f` is the first character of the `false` literal name.
      'f' => {
        if validate_false(&mut json_document, &mut scanner).is_err() {
          return json_document.errors;
        }
      }

      // Character `n` is the first character of the `null` literal name.
      'n' => {
        if validate_null(&mut json_document, &mut scanner).is_err() {
          return json_document.errors;
        }
      }

      // Parse the whole JSON number until a structural token or EOF.
      '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-' => {
        if validate_number(&mut json_document, &mut scanner).is_err() {
          return json_document.errors;
        }
      }

      // Parse the whole JSON string until a structural token or EOF.
      QUOTATION_MARK => {
        if validate_string(&mut json_document, &mut scanner).is_err() {
          return json_document.errors;
        }
      }

      // Check the validity of begin-array token, i.e. appropriate position,
      // nesting level, etc.
      // This operation checks only the begin-array character and returns here.
      BEGIN_ARRAY => {
        if validate_begin_array(&mut json_document, &mut scanner).is_err() {
          return json_document.errors;
        }
      }

      // Check the validity of end-array token, i.e. appropriate position,
      // nesting level, etc.
      // This operation checks only the end-array character and returns here.
      END_ARRAY => {
        if validate_end_array(&mut json_document, &mut scanner).is_err() {
          return json_document.errors;
        }
      }

      // Check the validity of name-separator token.
      // This operation checks only the name-separator character and returns here.
      VALUE_SEPARATOR => {
        if validate_value_separator(&mut json_document, &mut scanner).is_err() {
          return json_document.errors;
        }
      }

      // Check the validity of begin-object token, i.e. appropriate position,
      // nesting level, etc.
      // This operation checks only the begin-object character and returns here.
      BEGIN_OBJECT => {
        if validate_begin_object(&mut json_document, &mut scanner).is_err() {
          return json_document.errors;
        }
      }

      // Check the validity of end-object token, i.e. appropriate position,
      // nesting level, etc.
      // This operation checks only the end-object character and returns here.
      END_OBJECT => {
        if validate_end_object(&mut json_document, &mut scanner).is_err() {
          return json_document.errors;
        }
      }

      // Check the validity of name-separator token, i.e. appropriate position,
      // nesting level, etc.
      // This operation checks only the name-separator character and returns here.
      NAME_SEPARATOR => {
        if validate_name_separator(&mut json_document, &mut scanner).is_err() {
          return json_document.errors;
        }
      }

      // Invalid literal.
      _ => {
        let err = Error::new(ErrorType::E106, current_index, current_index + 1);
        json_document.errors.push(err);

        return json_document.errors;
      }
    }
  }

  // Prohibit empty JSON document.
  if json_document.last_parsed_token.is_none() {
    let last_parsed_index = scanner.current().index;
    // Empty JSON document.
    let err = Error::new(ErrorType::E100, last_parsed_index, last_parsed_index + 1);
    json_document.errors.push(err);
  }

  // Check if there are any tokens left in `json_document.stack` which denotes that some
  // nested structure has not terminated properly.
  if let Some(token) = json_document.stack.pop() {
    match token {
      StackTokens::BeginArray => {
        // Unterminated array.
        let last_parsed_index = scanner.current().index;
        let err = Error::new(ErrorType::E127, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);
      }

      StackTokens::BeginObject => {
        // Unterminated object.
        let last_parsed_index = scanner.current().index;
        let err = Error::new(ErrorType::E128, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);
      }

      StackTokens::NameSeparator => {
        // Invalid object member.
        let last_parsed_index = scanner.current().index;
        let err = Error::new(ErrorType::E141, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);
      }
    }
  }

  // In case there is any error, the resulted vector will contain
  // an Error instance, else will be empty in case of successful validation.
  json_document.errors
}
