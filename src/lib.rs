mod error;

use error::Error;

// Public exports
pub use error::ErrorType;

/// Validate a JSON document based on RFC 8259 latest standard.
pub fn validate(_code: &str) -> Vec<Error> {
  unimplemented!();
}
