mod error;
mod json;

use error::Error;
use json::general_tokens::*;
use json::{StackTokens, JSON};

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
pub use error::ErrorType;

/// Validate a JSON document based on RFC 8259 latest standard.
///
/// This function returns after catches one error, i.e. it is not error tolerant.
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
/// println!("{:#?}", errors); // => []
/// ```
///
/// ```rust
/// // Invalid `true` root value in JSON document.
/// let text: &str = "trua";
/// let errors = jsonprima::validate(&text);
/// println!("{:#?}", errors); // => [("E104", 0, 4)]
/// ```
pub fn validate(code: &str) -> Vec<Error> {
  let mut json_document = JSON::new(code);

  if json_document
    .iterator
    .peek()
    .filter(|(_, current_character)| current_character == &'\u{feff}')
    .is_some()
  {
    // Ignore byte order mark.
    json_document.iterator.next();
  }

  // Iterate over all characters and return a Result if there is any error.
  while let Some((current_index, current_character)) = json_document.iterator.next() {
    // Match the token type of the character (begin-array, horizontal-tab, etc).
    // We check the first character of a JSON value to determine
    // what value to validate, i.e. string, number, literal name, etc.
    match current_character {
      // Space character is always valid in a JSON document.
      SPACE | NEW_LINE | CARRIAGE_RETURN | HORIZONTAL_TAB => continue,

      // Character `t` is the first character of the `true` literal name.
      't' => {
        if validate_true(&mut json_document).is_err() {
          return json_document.errors;
        }
      }

      // Character `f` is the first character of the `false` literal name.
      'f' => {
        if validate_false(&mut json_document).is_err() {
          return json_document.errors;
        }
      }

      // Character `n` is the first character of the `null` literal name.
      'n' => {
        if validate_null(&mut json_document).is_err() {
          return json_document.errors;
        }
      }

      // Parse JSON number.
      '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-' => {
        if validate_number(&mut json_document).is_err() {
          return json_document.errors;
        }
      }

      // Parse JSON String
      QUOTATION_MARK => {
        if validate_string(&mut json_document).is_err() {
          return json_document.errors;
        }
      }

      BEGIN_ARRAY => {
        if validate_begin_array(&mut json_document).is_err() {
          return json_document.errors;
        }
      }

      END_ARRAY => {
        if validate_end_array(&mut json_document).is_err() {
          return json_document.errors;
        }
      }

      VALUE_SEPARATOR => {
        if validate_value_separator(&mut json_document).is_err() {
          return json_document.errors;
        }
      }

      BEGIN_OBJECT => {
        if validate_begin_object(&mut json_document).is_err() {
          return json_document.errors;
        }
      }

      END_OBJECT => {
        if validate_end_object(&mut json_document).is_err() {
          return json_document.errors;
        }
      }

      NAME_SEPARATOR => {
        if validate_name_separator(&mut json_document).is_err() {
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

  // In case we have not parsed any JSON value,
  // return empty JSON error.
  if json_document.last_parsed_token.is_none() {
    let last_parsed_index = json_document.iterator.current().index;
    // Empty JSON document.
    let err = Error::new(ErrorType::E100, last_parsed_index, last_parsed_index + 1);
    json_document.errors.push(err);
  }

  // Check if there are any json_document left in json_document.stack which denotes that some
  // nested structure has not terminated properly.
  if let Some(token) = json_document.stack.pop() {
    match token {
      StackTokens::BeginArray => {
        // Unterminated array.
        let last_parsed_index = json_document.iterator.current().index;
        let err = Error::new(ErrorType::E127, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);
      }

      StackTokens::BeginObject => {
        // Unterminated object.
        let last_parsed_index = json_document.iterator.current().index;
        let err = Error::new(ErrorType::E128, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);
      }

      StackTokens::NameSeparator => {
        // Invalid object member.
        let last_parsed_index = json_document.iterator.current().index;
        let err = Error::new(ErrorType::E141, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);
      }
    }
  }

  json_document.errors
}
