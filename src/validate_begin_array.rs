use crate::error::{Error, ErrorType};
use crate::json::{ParseTokens, StackTokens, JSON};

pub fn validate_begin_array(json_document: &mut JSON) -> Result<(), ()> {
  match &json_document.last_parsed_token {
    Some(last_parsed_token) => match last_parsed_token {
      ParseTokens::ValueSeparator => {
        match json_document.stack.last() {
          Some(token) => match token {
            StackTokens::BeginObject => {
              // Invalid use of array as object name.
              let last_parsed_index = json_document.iterator.current().index;
              let err =
                Error::new(ErrorType::E142, last_parsed_index, last_parsed_index + 1);
              json_document.errors.push(err);

              Err(())
            }

            StackTokens::BeginArray => {
              json_document.last_parsed_token = Some(ParseTokens::BeginArray);
              json_document.stack.push(StackTokens::BeginArray);
              Ok(())
            }

            _ => Ok(()), // unreachable
          },

          None => Ok(()), // unreachable
        }
      }

      ParseTokens::BeginArray => {
        json_document.last_parsed_token = Some(ParseTokens::BeginArray);
        json_document.stack.push(StackTokens::BeginArray);
        Ok(())
      }

      ParseTokens::NameSeparator => {
        json_document.object_has_valid_member = true;
        json_document.last_parsed_token = Some(ParseTokens::BeginArray);
        json_document.stack.pop();
        json_document.stack.push(StackTokens::BeginArray);
        Ok(())
      }

      ParseTokens::BeginObject => {
        // Illegal begin-array after JSON value.
        let last_parsed_index = json_document.iterator.current().index;
        let err = Error::new(ErrorType::E125, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);

        Err(())
      }

      _ => {
        // Illegal begin-array after JSON value.
        let last_parsed_index = json_document.iterator.current().index;
        let err = Error::new(ErrorType::E125, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);

        Err(())
      }
    },

    None => {
      json_document.last_parsed_token = Some(ParseTokens::BeginArray);
      json_document.stack.push(StackTokens::BeginArray);
      json_document.root_value_parsed = true;
      Ok(())
    }
  }
}
