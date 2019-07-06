mod error;
mod tokens;

use error::Error;
use tokens::Tokens;

// Public exports
pub use error::ErrorType;

/// Validate a JSON document based on RFC 8259 latest standard.
///
/// This function returns after catches one error, i.e. it is not error tolerant.
///
/// **Notes**
/// - There is no limit to the size of the provided text to validate.
/// - There is no limit to the nesting of the JSON document.
///
/// **Examples**
///
/// ```rust
/// // A valid JSON document.
/// let text: &str = "[true, false]";
/// let errors = jsonprima::validate(&text);
/// println!("{:#?}", errors); // => []
/// ```
pub fn validate(code: &str) -> Vec<Error> {
  let mut tokens = Tokens::new(code.chars().enumerate().peekable());

  // Iterate over all characters and return a Result if there is any error.
  while let Some((current_index, current_character)) = tokens.iterator.next() {
    // Save the current index and character to tokens struct.
    tokens.current_iterator_index = current_index;
    tokens.current_iterator_character = current_character;

    // Match the token type of the character (begin-array, horizontal-tab, etc).
    // We check the first character of a JSON value to determine
    // what value to validate, i.e. string, number, literal name, etc.
    match current_character {
      // Invalid literal.
      _ => {
        let err = Error::new(ErrorType::E106, current_index, current_index + 1);
        tokens.errors.push(err);

        return tokens.errors;
      }
    }
  }

  tokens.errors
}
