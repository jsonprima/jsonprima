use crate::error::{Error, ErrorType};
use crate::json::{ParseTokens, StackTokens, JSON};

// Helper macro to test if the next entry in json_document.iterator
// match expected_character.
macro_rules! next {
  ($json_document:ident, $expected_character:expr, $index_start:expr) => {
    let result = $json_document
      .iterator
      .next()
      .ok_or_else(|| {
        // No more graphemes to parse.
        let err = Error::new(
          ErrorType::E104,
          $index_start,
          $json_document.iterator.current().index + 1,
        );

        $json_document.errors.push(err);
        ()
      })
      .and_then(|(next_index, next_character)| {
        if next_character != $expected_character {
          // Invalid character in literal name.
          let err = Error::new(ErrorType::E105, $index_start, next_index + 1);
          $json_document.errors.push(err);
          Err(())
        } else {
          Ok(())
        }
      });

    if result.is_err() {
      return result;
    }
  };
}

fn validate(json_document: &mut JSON) -> Result<(), ()> {
  // Save the position of the first character.
  // This will help us set a range that will highlight the whole incorrect value
  // in case of an error.
  //
  // Example:
  //
  // ```rust
  // // Invalid `falsA` root value in JSON document.
  // let text: &str = "falsA";
  // let errors = jsonprima::validate(&text);
  // println!("{:#?}", errors); // => [("E104", 0, 5)]
  // ```
  let index_start = json_document.iterator.current().index;

  next!(json_document, 'a', index_start);
  next!(json_document, 'l', index_start);
  next!(json_document, 's', index_start);
  next!(json_document, 'e', index_start);

  // If none of the macro invocations returned error,
  // we have successfully validate the value.
  json_document.last_parsed_token = Some(ParseTokens::False);

  Ok(())
}

pub fn validate_false(json_document: &mut JSON) -> Result<(), ()> {
  match &json_document.last_parsed_token {
    Some(last_parsed_token) => match last_parsed_token {
      ParseTokens::BeginArray => validate(json_document),

      ParseTokens::ValueSeparator => {
        if json_document.stack.last().unwrap() == &StackTokens::BeginObject {
          // Invalid use of false as object name.
          let last_parsed_index = json_document.iterator.current().index;
          let err = Error::new(ErrorType::E139, last_parsed_index, last_parsed_index + 1);
          json_document.errors.push(err);

          Err(())
        } else {
          validate(json_document)
        }
      }

      ParseTokens::BeginObject => {
        // Invalid use of false as object name.
        let last_parsed_index = json_document.iterator.current().index;
        let err = Error::new(ErrorType::E139, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);

        Err(())
      }

      ParseTokens::NameSeparator => {
        json_document.stack.pop();
        json_document.object_has_valid_member = true;

        validate(json_document)
      }

      // Illegal character "f" after structural token. Expected comma or colon.
      _ => {
        let last_parsed_index = json_document.iterator.current().index;
        let err = Error::new(ErrorType::E107, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);

        Err(())
      }
    },

    None => {
      json_document.root_value_parsed = true;
      validate(json_document)
    }
  }
}
