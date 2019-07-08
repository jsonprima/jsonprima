use crate::error::{Error, ErrorType};
use crate::tokens::{ParseTokens, StackTokens, Tokens};

pub fn validate_name_separator(tokens: &mut Tokens) -> Result<(), ()> {
  match &tokens.last_parsed_token {
    Some(token) => match token {
      ParseTokens::EndArray
      | ParseTokens::EndObject
      | ParseTokens::String
      | ParseTokens::Number
      | ParseTokens::True
      | ParseTokens::False
      | ParseTokens::Null => {
        match tokens.stack.last() {
          Some(token) => match token {
            StackTokens::BeginObject => {
              // avoid double colon
              if tokens.object_has_valid_member {
                // Invalid use of colon.
                let last_parsed_index = tokens.current_iterator_index;
                let err =
                  Error::new(ErrorType::E136, last_parsed_index, last_parsed_index + 1);
                tokens.errors.push(err);

                Err(())
              } else {
                tokens.last_parsed_token = Some(ParseTokens::NameSeparator);
                tokens.stack.push(StackTokens::NameSeparator);
                Ok(())
              }
            }

            _ => {
              // Invalid use of colon.
              let last_parsed_index = tokens.current_iterator_index;
              let err =
                Error::new(ErrorType::E136, last_parsed_index, last_parsed_index + 1);
              tokens.errors.push(err);

              Err(())
            }
          },

          None => {
            // Invalid use of colon.
            let last_parsed_index = tokens.current_iterator_index;
            let err =
              Error::new(ErrorType::E136, last_parsed_index, last_parsed_index + 1);
            tokens.errors.push(err);

            Err(())
          }
        }
      }

      _ => {
        // Invalid use of colon.
        let last_parsed_index = tokens.current_iterator_index;
        let err = Error::new(ErrorType::E136, last_parsed_index, last_parsed_index + 1);
        tokens.errors.push(err);

        Err(())
      }
    },

    None => {
      // Invalid use of colon.
      let last_parsed_index = tokens.current_iterator_index;
      let err = Error::new(ErrorType::E136, last_parsed_index, last_parsed_index + 1);
      tokens.errors.push(err);

      Err(())
    }
  }
}
