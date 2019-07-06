mod error;
mod tokens;

use error::Error;
use tokens::general_tokens::*;
use tokens::Tokens;

mod validate_true;
use validate_true::validate_true;

mod validate_false;
use validate_false::validate_false;

mod validate_null;
use validate_null::validate_null;

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
  let mut tokens = Tokens::new(code.chars().enumerate().peekable());

  if tokens
    .iterator
    .peek()
    .filter(|(_, current_character)| current_character == &'\u{feff}')
    .is_some()
  {
    // Ignore byte order mark.
    tokens.iterator.next();
  }

  // Iterate over all characters and return a Result if there is any error.
  while let Some((current_index, current_character)) = tokens.iterator.next() {
    // Save the current index and character to tokens struct.
    tokens.current_iterator_index = current_index;
    tokens.current_iterator_character = current_character;

    // Match the token type of the character (begin-array, horizontal-tab, etc).
    // We check the first character of a JSON value to determine
    // what value to validate, i.e. string, number, literal name, etc.
    match current_character {
      // Space character is always valid in a JSON document.
      SPACE | NEW_LINE | CARRIAGE_RETURN | HORIZONTAL_TAB => continue,

      // Character `t` is the first character of the `true` literal name.
      't' => {
        if validate_true(&mut tokens).is_err() {
          return tokens.errors;
        }
      }

      // Character `f` is the first character of the `false` literal name.
      'f' => {
        if validate_false(&mut tokens).is_err() {
          return tokens.errors;
        }
      }

      // Character `n` is the first character of the `null` literal name.
      'n' => {
        if validate_null(&mut tokens).is_err() {
          return tokens.errors;
        }
      }

      // Invalid literal.
      _ => {
        let err = Error::new(ErrorType::E106, current_index, current_index + 1);
        tokens.errors.push(err);

        return tokens.errors;
      }
    }
  }

  // In case we have not parsed any JSON value,
  // return empty JSON error.
  if tokens.last_parsed_token.is_none() {
    let last_parsed_index = tokens.current_iterator_index;
    // Empty JSON document.
    let err = Error::new(ErrorType::E100, last_parsed_index, last_parsed_index + 1);
    tokens.errors.push(err);
  }

  tokens.errors
}
