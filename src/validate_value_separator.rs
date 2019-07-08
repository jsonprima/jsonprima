use crate::error::{Error, ErrorType};
use crate::json::{ParseTokens, StackTokens, JSON};

pub fn validate_value_separator(json_document: &mut JSON) -> Result<(), ()> {
  match &json_document.last_parsed_token {
    Some(token) => {
      match token {
        ParseTokens::Null
        | ParseTokens::Number
        | ParseTokens::True
        | ParseTokens::False
        | ParseTokens::String => {
          if json_document.root_value_parsed && json_document.stack.is_empty() {
            // Invalid use of comma after document root value.
            let last_parsed_index = json_document.iterator.current().index;
            let err =
              Error::new(ErrorType::E124, last_parsed_index, last_parsed_index + 1);
            json_document.errors.push(err);

            Err(())
          } else {
            json_document.last_parsed_token = Some(ParseTokens::ValueSeparator);
            Ok(())
          }
        }

        ParseTokens::EndArray | ParseTokens::EndObject => {
          if !json_document.stack.is_empty() {
            match json_document.stack.last().unwrap() {
              StackTokens::BeginArray | StackTokens::BeginObject => {
                json_document.last_parsed_token = Some(ParseTokens::ValueSeparator);
                Ok(())
              }

              _ => Ok(()),
            }
          } else {
            // Invalid use of comma after document root value.
            let last_parsed_index = json_document.iterator.current().index;
            let err =
              Error::new(ErrorType::E124, last_parsed_index, last_parsed_index + 1);
            json_document.errors.push(err);

            Err(())
          }
        }

        _ => {
          // Unexpected comma after structural token.
          let last_parsed_index = json_document.iterator.current().index;
          let err = Error::new(ErrorType::E123, last_parsed_index, last_parsed_index + 1);
          json_document.errors.push(err);

          Err(())
        }
      }
    }

    None => {
      // Unexpected comma at the start of JSON document.
      let last_parsed_index = json_document.iterator.current().index;
      let err = Error::new(ErrorType::E122, last_parsed_index, last_parsed_index + 1);
      json_document.errors.push(err);

      Err(())
    }
  }
}
