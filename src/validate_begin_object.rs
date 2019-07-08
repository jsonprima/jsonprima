use crate::error::{Error, ErrorType};
use crate::json::{ParseTokens, StackTokens, JSON};
use crate::scanner::Scanner;

pub fn validate_begin_object(
  json_document: &mut JSON,
  scanner: &mut Scanner,
) -> Result<(), ()> {
  match &json_document.last_parsed_token {
    Some(last_parsed_token) => match last_parsed_token {
      ParseTokens::ValueSeparator => {
        match json_document.stack.last() {
          Some(token) => match token {
            StackTokens::BeginObject => {
              // Invalid use of object as object name.
              let last_parsed_index = scanner.current().index;
              let err =
                Error::new(ErrorType::E143, last_parsed_index, last_parsed_index + 1);
              json_document.errors.push(err);

              Err(())
            }

            StackTokens::BeginArray => {
              json_document.object_has_valid_member = true;
              json_document.last_parsed_token = Some(ParseTokens::BeginObject);
              json_document.stack.push(StackTokens::BeginObject);
              Ok(())
            }

            _ => Ok(()),
          },

          None => Ok(()), // unreachable
        }
      }

      ParseTokens::BeginArray => {
        json_document.object_has_valid_member = true;
        json_document.last_parsed_token = Some(ParseTokens::BeginObject);
        json_document.stack.push(StackTokens::BeginObject);
        Ok(())
      }

      ParseTokens::NameSeparator => {
        json_document.object_has_valid_member = true;
        json_document.last_parsed_token = Some(ParseTokens::BeginObject);
        json_document.stack.pop();
        json_document.stack.push(StackTokens::BeginObject);
        Ok(())
      }

      _ => {
        // Illegal begin-object after JSON value.
        let last_parsed_index = scanner.current().index;
        let err = Error::new(ErrorType::E130, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);

        Err(())
      }
    },

    None => {
      json_document.object_has_valid_member = true;
      json_document.last_parsed_token = Some(ParseTokens::BeginObject);
      json_document.stack.push(StackTokens::BeginObject);
      json_document.root_value_parsed = true;
      Ok(())
    }
  }
}
