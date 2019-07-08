use crate::error::{Error, ErrorType};
use crate::json::{ParseTokens, StackTokens, JSON};

pub fn validate_end_array(json_document: &mut JSON) -> Result<(), ()> {
  match &json_document.last_parsed_token {
    Some(last_parsed_token) => match last_parsed_token {
      ParseTokens::BeginArray
      | ParseTokens::EndArray
      | ParseTokens::True
      | ParseTokens::False
      | ParseTokens::Null
      | ParseTokens::Number
      | ParseTokens::String
      | ParseTokens::EndObject => {
        match json_document.stack.last() {
          Some(token) => match token {
            StackTokens::BeginArray => {
              json_document.last_parsed_token = Some(ParseTokens::EndArray);
              json_document.stack.pop();
              Ok(())
            }

            _ => {
              // Illegal end-array. No begin-array match.
              let last_parsed_index = json_document.iterator.current().index;
              let err =
                Error::new(ErrorType::E126, last_parsed_index, last_parsed_index + 1);
              json_document.errors.push(err);

              Err(())
            }
          },

          None => {
            // Illegal end-array. No begin-array match.
            let last_parsed_index = json_document.iterator.current().index;
            let err =
              Error::new(ErrorType::E126, last_parsed_index, last_parsed_index + 1);
            json_document.errors.push(err);

            Err(())
          }
        }
      }

      ParseTokens::ValueSeparator => {
        // Illegal end-array after comma.
        let last_parsed_index = json_document.iterator.current().index;
        let err = Error::new(ErrorType::E129, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);

        Err(())
      }

      ParseTokens::NameSeparator => {
        // Illegal end-array after colon.
        let last_parsed_index = json_document.iterator.current().index;
        let err = Error::new(ErrorType::E133, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);

        Err(())
      }

      _ => {
        // Illegal end-array. No begin-array match.
        let last_parsed_index = json_document.iterator.current().index;
        let err = Error::new(ErrorType::E126, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);

        Err(())
      }
    },

    None => {
      // Illegal end-array. No begin-array match.
      let last_parsed_index = json_document.iterator.current().index;
      let err = Error::new(ErrorType::E126, last_parsed_index, last_parsed_index + 1);
      json_document.errors.push(err);

      Err(())
    }
  }
}
