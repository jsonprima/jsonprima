use crate::error::{Error, ErrorType};
use crate::tokens::{ParseTokens, StackTokens, Tokens};

pub fn validate_begin_object(tokens: &mut Tokens) -> Result<(), ()> {
  match &tokens.last_parsed_token {
    Some(last_parsed_token) => match last_parsed_token {
      ParseTokens::ValueSeparator => {
        match tokens.stack.last() {
          Some(token) => match token {
            StackTokens::BeginObject => {
              // Invalid use of object as object name.
              let last_parsed_index = tokens.current_iterator_index;
              let err =
                Error::new(ErrorType::E143, last_parsed_index, last_parsed_index + 1);
              tokens.errors.push(err);

              Err(())
            }

            StackTokens::BeginArray => {
              tokens.object_has_valid_member = true;
              tokens.last_parsed_token = Some(ParseTokens::BeginObject);
              tokens.stack.push(StackTokens::BeginObject);
              Ok(())
            }

            _ => Ok(()),
          },

          None => Ok(()), // unreachable
        }
      }

      ParseTokens::BeginArray => {
        tokens.object_has_valid_member = true;
        tokens.last_parsed_token = Some(ParseTokens::BeginObject);
        tokens.stack.push(StackTokens::BeginObject);
        Ok(())
      }

      ParseTokens::NameSeparator => {
        tokens.object_has_valid_member = true;
        tokens.last_parsed_token = Some(ParseTokens::BeginObject);
        tokens.stack.pop();
        tokens.stack.push(StackTokens::BeginObject);
        Ok(())
      }

      _ => {
        // Illegal begin-object after JSON value.
        let last_parsed_index = tokens.current_iterator_index;
        let err = Error::new(ErrorType::E130, last_parsed_index, last_parsed_index + 1);
        tokens.errors.push(err);

        Err(())
      }
    },

    None => {
      tokens.object_has_valid_member = true;
      tokens.last_parsed_token = Some(ParseTokens::BeginObject);
      tokens.stack.push(StackTokens::BeginObject);
      tokens.root_value_parsed = true;
      Ok(())
    }
  }
}
