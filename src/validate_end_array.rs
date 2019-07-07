use crate::error::{Error, ErrorType};
use crate::tokens::{ParseTokens, StackTokens, Tokens};

pub fn validate_end_array(tokens: &mut Tokens) -> Result<(), ()> {
  match &tokens.last_parsed_token {
    Some(last_parsed_token) => match last_parsed_token {
      ParseTokens::BeginArray | ParseTokens::EndArray => {
        match tokens.stack.last() {
          Some(token) => match token {
            StackTokens::BeginArray => {
              tokens.last_parsed_token = Some(ParseTokens::EndArray);
              tokens.stack.pop();
              Ok(())
            }
          },

          None => {
            // Illegal end-array. No begin-array match.
            let last_parsed_index = tokens.current_iterator_index;
            let err =
              Error::new(ErrorType::E126, last_parsed_index, last_parsed_index + 1);
            tokens.errors.push(err);

            Err(())
          }
        }
      }

      _ => {
        // Illegal end-array. No begin-array match.
        let last_parsed_index = tokens.current_iterator_index;
        let err = Error::new(ErrorType::E126, last_parsed_index, last_parsed_index + 1);
        tokens.errors.push(err);

        Err(())
      }
    },

    None => {
      // Illegal end-array. No begin-array match.
      let last_parsed_index = tokens.current_iterator_index;
      let err = Error::new(ErrorType::E126, last_parsed_index, last_parsed_index + 1);
      tokens.errors.push(err);

      Err(())
    }
  }
}
