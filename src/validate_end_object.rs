use crate::error::{Error, ErrorType};
use crate::json::{ParseTokens, StackTokens, JSON};

pub fn validate_end_object(json_document: &mut JSON) -> Result<(), ()> {
  match &json_document.last_parsed_token {
    Some(last_parsed_token) => match last_parsed_token {
      ParseTokens::EndObject
      | ParseTokens::EndArray
      | ParseTokens::BeginObject
      | ParseTokens::String
      | ParseTokens::Number
      | ParseTokens::Null
      | ParseTokens::True
      | ParseTokens::False => {
        match json_document.stack.last() {
          Some(token) => match token {
            StackTokens::BeginObject => {
              if json_document.object_has_valid_member {
                json_document.last_parsed_token = Some(ParseTokens::EndObject);
                json_document.stack.pop();
                Ok(())
              } else {
                // Invalid object member. Member value does not exist.
                let last_parsed_index = json_document.current_iterator_index;
                let err =
                  Error::new(ErrorType::E135, last_parsed_index, last_parsed_index + 1);
                json_document.errors.push(err);

                Err(())
              }
            }

            _ => {
              // Illegal end-object. No begin-object match.
              let last_parsed_index = json_document.current_iterator_index;
              let err =
                Error::new(ErrorType::E131, last_parsed_index, last_parsed_index + 1);
              json_document.errors.push(err);

              Err(())
            }
          },

          None => {
            // Illegal end-object. No begin-object match.
            let last_parsed_index = json_document.current_iterator_index;
            let err =
              Error::new(ErrorType::E131, last_parsed_index, last_parsed_index + 1);
            json_document.errors.push(err);

            Err(())
          }
        }
      }

      ParseTokens::ValueSeparator => {
        // Illegal end-object after comma.
        let last_parsed_index = json_document.current_iterator_index;
        let err = Error::new(ErrorType::E132, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);

        Err(())
      }

      ParseTokens::NameSeparator => {
        // Illegal end-object after colon.
        let last_parsed_index = json_document.current_iterator_index;
        let err = Error::new(ErrorType::E134, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);

        Err(())
      }

      _ => {
        // Illegal end-object. No begin-object match.
        let last_parsed_index = json_document.current_iterator_index;
        let err = Error::new(ErrorType::E131, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);

        Err(())
      }
    },

    None => {
      // Illegal end-object. No begin-object match.
      let last_parsed_index = json_document.current_iterator_index;
      let err = Error::new(ErrorType::E131, last_parsed_index, last_parsed_index + 1);
      json_document.errors.push(err);

      Err(())
    }
  }
}
