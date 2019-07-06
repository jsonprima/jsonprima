mod error;
mod tokens;

use error::Error;
use tokens::Tokens;

// Public exports
pub use error::ErrorType;

/// Validate a JSON document based on RFC 8259 latest standard.
pub fn validate(code: &str) -> Vec<Error> {
  let tokens = Tokens::new(code.chars().enumerate().peekable());

  tokens.errors
}
