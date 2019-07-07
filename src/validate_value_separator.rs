use crate::error::{Error, ErrorType};
use crate::tokens::{ParseTokens, StackTokens, Tokens};

pub fn validate_value_separator(tokens: &mut Tokens) -> Result<(), ()> {
  match &tokens.last_parsed_token {
    Some(token) => {
      match token {
        ParseTokens::Null
        | ParseTokens::Number
        | ParseTokens::True
        | ParseTokens::False
        | ParseTokens::String => {
          if tokens.root_value_parsed && tokens.stack.is_empty() {
            // Invalid use of comma after document root value.
            let last_parsed_index = tokens.current_iterator_index;
            let err =
              Error::new(ErrorType::E124, last_parsed_index, last_parsed_index + 1);
            tokens.errors.push(err);

            Err(())
          } else {
            tokens.last_parsed_token = Some(ParseTokens::ValueSeparator);
            Ok(())
          }
        }

        ParseTokens::EndArray => {
          if !tokens.stack.is_empty() {
            match tokens.stack.last().unwrap() {
              StackTokens::BeginArray => {
                tokens.last_parsed_token = Some(ParseTokens::ValueSeparator);
                Ok(())
              }

              _ => Ok(()),
            }
          } else {
            // Invalid use of comma after document root value.
            let last_parsed_index = tokens.current_iterator_index;
            let err =
              Error::new(ErrorType::E124, last_parsed_index, last_parsed_index + 1);
            tokens.errors.push(err);

            Err(())
          }
        }

        _ => {
          // Unexpected comma after structural token.
          let last_parsed_index = tokens.current_iterator_index;
          let err = Error::new(ErrorType::E123, last_parsed_index, last_parsed_index + 1);
          tokens.errors.push(err);

          Err(())
        }
      }
    }

    None => {
      // Unexpected comma at the start of JSON document.
      let last_parsed_index = tokens.current_iterator_index;
      let err = Error::new(ErrorType::E122, last_parsed_index, last_parsed_index + 1);
      tokens.errors.push(err);

      Err(())
    }
  }
}
