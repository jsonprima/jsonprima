use crate::error::{Error, ErrorType};
use crate::json::{ParseTokens, StackTokens, JSON};
use crate::scanner::Scanner;

pub fn validate_name_separator(
  json_document: &mut JSON,
  scanner: &mut Scanner,
) -> Result<(), ()> {
  match &json_document.last_parsed_token {
    Some(token) => match token {
      ParseTokens::EndArray
      | ParseTokens::EndObject
      | ParseTokens::String
      | ParseTokens::Number
      | ParseTokens::True
      | ParseTokens::False
      | ParseTokens::Null => {
        match json_document.stack.last() {
          Some(token) => match token {
            StackTokens::BeginObject => {
              // avoid double colon
              if json_document.object_has_valid_member {
                // Invalid use of colon.
                let last_parsed_index = scanner.current().index;
                let err =
                  Error::new(ErrorType::E136, last_parsed_index, last_parsed_index + 1);
                json_document.errors.push(err);

                Err(())
              } else {
                json_document.last_parsed_token = Some(ParseTokens::NameSeparator);
                json_document.stack.push(StackTokens::NameSeparator);
                Ok(())
              }
            }

            _ => {
              // Invalid use of colon.
              let last_parsed_index = scanner.current().index;
              let err =
                Error::new(ErrorType::E136, last_parsed_index, last_parsed_index + 1);
              json_document.errors.push(err);

              Err(())
            }
          },

          None => {
            // Invalid use of colon.
            let last_parsed_index = scanner.current().index;
            let err =
              Error::new(ErrorType::E136, last_parsed_index, last_parsed_index + 1);
            json_document.errors.push(err);

            Err(())
          }
        }
      }

      _ => {
        // Invalid use of colon.
        let last_parsed_index = scanner.current().index;
        let err = Error::new(ErrorType::E136, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);

        Err(())
      }
    },

    None => {
      // Invalid use of colon.
      let last_parsed_index = scanner.current().index;
      let err = Error::new(ErrorType::E136, last_parsed_index, last_parsed_index + 1);
      json_document.errors.push(err);

      Err(())
    }
  }
}
