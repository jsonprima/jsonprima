use crate::error::{Error, ErrorType};
use crate::json::{ParseTokens, StackTokens, JSON};
use crate::scanner::Scanner;

// Helper macro to test if the next entry in scanner
// match expected_character.
macro_rules! next {
  ($json_document:ident, $scanner:ident, $expected_character:expr, $index_start:expr) => {
    let result = $scanner
      .next()
      .ok_or_else(|| {
        // No more graphemes to parse.
        let err = Error::new(ErrorType::E104, $index_start, $scanner.current().index + 1);

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

fn validate(json_document: &mut JSON, scanner: &mut Scanner) -> Result<(), ()> {
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
  let index_start = scanner.current().index;

  next!(json_document, scanner, 'u', index_start);
  next!(json_document, scanner, 'l', index_start);
  next!(json_document, scanner, 'l', index_start);

  // If none of the macro invocations returned error,
  // we have successfully validate the value.
  json_document.last_parsed_token = Some(ParseTokens::Null);

  Ok(())
}

pub fn validate_null(json_document: &mut JSON, scanner: &mut Scanner) -> Result<(), ()> {
  match &json_document.last_parsed_token {
    Some(last_parsed_token) => match last_parsed_token {
      ParseTokens::BeginArray => validate(json_document, scanner),

      ParseTokens::ValueSeparator => {
        if json_document.stack.last().unwrap() == &StackTokens::BeginObject {
          // Invalid use of null as object name.
          let last_parsed_index = scanner.current().index;
          let err = Error::new(ErrorType::E140, last_parsed_index, last_parsed_index + 1);
          json_document.errors.push(err);

          Err(())
        } else {
          validate(json_document, scanner)
        }
      }

      ParseTokens::BeginObject => {
        // Invalid use of null as object name.
        let last_parsed_index = scanner.current().index;
        let err = Error::new(ErrorType::E140, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);

        Err(())
      }

      ParseTokens::NameSeparator => {
        json_document.stack.pop();
        json_document.object_has_valid_member = true;

        validate(json_document, scanner)
      }

      // Illegal character "n" after structural token. Expected comma or colon.
      _ => {
        let last_parsed_index = scanner.current().index;
        let err = Error::new(ErrorType::E108, last_parsed_index, last_parsed_index + 1);
        json_document.errors.push(err);

        Err(())
      }
    },

    None => {
      json_document.root_value_parsed = true;
      validate(json_document, scanner)
    }
  }
}
