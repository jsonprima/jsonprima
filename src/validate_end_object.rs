use crate::error::{Error, ErrorType};
use crate::tokens::{ParseTokens, StackTokens, Tokens};

pub fn validate_end_object(tokens: &mut Tokens) -> Result<(), ()> {
  match &tokens.last_parsed_token {
    Some(last_parsed_token) => match last_parsed_token {
      ParseTokens::EndObject
      | ParseTokens::EndArray
      | ParseTokens::BeginObject
      | ParseTokens::String
      | ParseTokens::Number
      | ParseTokens::Null
      | ParseTokens::True
      | ParseTokens::False => {
        match tokens.stack.last() {
          Some(token) => match token {
            StackTokens::BeginObject => {
              if tokens.object_has_valid_member {
                tokens.last_parsed_token = Some(ParseTokens::EndObject);
                tokens.stack.pop();
                Ok(())
              } else {
                // Invalid object member. Member value does not exist.
                let last_parsed_index = tokens.current_iterator_index;
                let err =
                  Error::new(ErrorType::E135, last_parsed_index, last_parsed_index + 1);
                tokens.errors.push(err);

                Err(())
              }
            }

            _ => {
              // Illegal end-object. No begin-object match.
              let last_parsed_index = tokens.current_iterator_index;
              let err =
                Error::new(ErrorType::E131, last_parsed_index, last_parsed_index + 1);
              tokens.errors.push(err);

              Err(())
            }
          },

          None => {
            // Illegal end-object. No begin-object match.
            let last_parsed_index = tokens.current_iterator_index;
            let err =
              Error::new(ErrorType::E131, last_parsed_index, last_parsed_index + 1);
            tokens.errors.push(err);

            Err(())
          }
        }
      }

      ParseTokens::ValueSeparator => {
        // Illegal end-object after comma.
        let last_parsed_index = tokens.current_iterator_index;
        let err = Error::new(ErrorType::E132, last_parsed_index, last_parsed_index + 1);
        tokens.errors.push(err);

        Err(())
      }

      ParseTokens::NameSeparator => {
        // Illegal end-object after colon.
        let last_parsed_index = tokens.current_iterator_index;
        let err = Error::new(ErrorType::E134, last_parsed_index, last_parsed_index + 1);
        tokens.errors.push(err);

        Err(())
      }

      _ => {
        // Illegal end-object. No begin-object match.
        let last_parsed_index = tokens.current_iterator_index;
        let err = Error::new(ErrorType::E131, last_parsed_index, last_parsed_index + 1);
        tokens.errors.push(err);

        Err(())
      }
    },

    None => {
      // Illegal end-object. No begin-object match.
      let last_parsed_index = tokens.current_iterator_index;
      let err = Error::new(ErrorType::E131, last_parsed_index, last_parsed_index + 1);
      tokens.errors.push(err);

      Err(())
    }
  }
}
