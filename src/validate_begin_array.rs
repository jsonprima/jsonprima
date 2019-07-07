use crate::error::{Error, ErrorType};
use crate::tokens::{ParseTokens, StackTokens, Tokens};

pub fn validate_begin_array(tokens: &mut Tokens) -> Result<(), ()> {
  match &tokens.last_parsed_token {
    Some(last_parsed_token) => match last_parsed_token {
      _ => {
        // Illegal begin-array after JSON value.
        let last_parsed_index = tokens.current_iterator_index;
        let err = Error::new(ErrorType::E125, last_parsed_index, last_parsed_index + 1);
        tokens.errors.push(err);

        Err(())
      }
    },

    None => {
      tokens.last_parsed_token = Some(ParseTokens::BeginArray);
      tokens.stack.push(StackTokens::BeginArray);
      tokens.root_value_parsed = true;
      Ok(())
    }
  }
}
