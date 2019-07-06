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
  let tokens = Tokens::new(code.chars().enumerate().peekable());

  tokens.errors
}
