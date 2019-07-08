use crate::error::{Error, ErrorType};
use crate::json::{general_tokens::*, ParseTokens, StackTokens, JSON};
use crate::scanner::Scanner;

// Given the json_document structure where the iterator index is in the
// start of a Unicode escape sequence values, i.e. X in \uXXXX,
// get the decimal representation of the escape sequence.
fn parse_escape_sequence(
  index_start: usize,
  scanner: &mut Scanner,
) -> Result<u32, Error> {
  // The decimal representation of the Unicode escape sequence.
  let mut uffff: u32 = 0;

  // Unicode escape sequence is defined
  // as \uxxxx, where x is ASCII Hex value.
  for _ in 0..4 {
    let next = scanner.next();

    if next.is_none() {
      return Err(Error::new(
        ErrorType::E104,
        index_start,
        scanner.current().index + 1,
      ));
    }

    if !scanner.current().character.is_ascii_hexdigit() {
      // Invalid character found in Unicode escape sequence.
      return Err(Error::new(
        ErrorType::E117,
        index_start,
        scanner.current().index + 1,
      ));
    }

    let hex = scanner.current().character.to_digit(16).unwrap();
    uffff = uffff * 16 + hex
  }

  // Return the decimal value of the Unicode escape sequence.
  Ok(uffff)
}

fn validate(json_document: &mut JSON, scanner: &mut Scanner) -> Result<(), ()> {
  // Save the position of the first character.
  // This will help us set a range that will highlight the whole incorrect value
  // in case of an error.
  //
  // Example:
  //
  // ```rust
  // // Invalid `"unterminated` root value in JSON document.
  // let text: &str = "\"unterminated";
  // let errors = jsonprima::validate(&text);
  // println!("{:#?}", errors); // => [("E104", 0, 13)]
  // ```
  let index_start = scanner.current().index;

  // Parse all characters as string values until quotation mark.
  loop {
    scanner.next().ok_or_else(|| {
      // No more characters to parse.
      let err = Error::new(ErrorType::E104, index_start, scanner.current().index + 1);
      json_document.errors.push(err);
      ()
    })?;

    // End of string reached. We have successfully parse the JSON string.
    if scanner.current().character == '"' {
      json_document.last_parsed_token = Some(ParseTokens::String);
      return Ok(());
    }

    // In case the character is not closing quotation mark and exists,
    // validate it based on the rules of RFC 8259.

    match scanner.current().character {
      '\\' => {
        // Start of escape character.
        // Read the next character to find out more.
        scanner.next().ok_or_else(|| {
          // No more characters to parse.
          let err = Error::new(ErrorType::E104, index_start, scanner.current().index + 1);
          json_document.errors.push(err);
          ()
        })?;

        match scanner.current().character {
          // Valid escape character sequence.
          '/' | '\\' | '"' | 'b' | 'f' | 'n' | 'r' | 't' => continue,

          // Start of Unicode escape sequence.
          'u' => {
            // Unicode escape sequences can form a surrogate pair.
            // Parse the first escape sequence
            // and if is invalid we assume that is part of
            // a surrogate pair and we parse the next one.
            // If the second escape sequence forms an invalid
            // surrogate pair then we return with an error.
            // If the first escape sequence is valid, then we
            // do not have to parse the second, as it is not a
            // surrogate pair.
            let high_surrogate: u32 = parse_escape_sequence(index_start, scanner)
              .or_else(|err| {
                json_document.errors.push(err);
                Err(())
              })?;

            // Check parsed Unicode value.
            match std::char::from_u32(high_surrogate) {
              // We successfully parsed the Unicode
              // escaped sequence, no surrogate pair.
              Some(_) => continue,

              None => {
                // We couldn't parse the Unicode escape
                // sequence. This most likely means that
                // is part of a surrogate pair.

                // Start parsing the next Unicode escape sequence,
                // as low surrogate pair.
                scanner.next().ok_or_else(|| {
                  // Invalid Unicode escape sequence in second surrogate pair.
                  let err =
                    Error::new(ErrorType::E119, index_start, scanner.current().index + 1);
                  json_document.errors.push(err);
                  ()
                })?;

                if scanner.current().character != '\\' {
                  let err =
                    Error::new(ErrorType::E119, index_start, scanner.current().index + 1);
                  json_document.errors.push(err);

                  return Err(());
                }

                scanner.next().ok_or_else(|| {
                  // Invalid Unicode escape sequence in second surrogate pair.
                  let err =
                    Error::new(ErrorType::E119, index_start, scanner.current().index + 1);
                  json_document.errors.push(err);
                  ()
                })?;

                if scanner.current().character != 'u' {
                  let err =
                    Error::new(ErrorType::E119, index_start, scanner.current().index + 1);
                  json_document.errors.push(err);

                  return Err(());
                }

                let low_surrogate: u32 = parse_escape_sequence(index_start, scanner)
                  .or_else(|err| {
                    json_document.errors.push(err);
                    Err(())
                  })?;

                // Borrowed from https://stackoverflow.com/a/23920015
                let unicode_value = (high_surrogate << 10) + low_surrogate - 0x35f_dc00;

                std::char::from_u32(unicode_value).ok_or_else(|| {
                  // Invalid Unicode character in JSON string.
                  let err =
                    Error::new(ErrorType::E118, index_start, scanner.current().index + 1);
                  json_document.errors.push(err);
                  ()
                })?;
              }
            }
          }

          HORIZONTAL_TAB | NEW_LINE | CARRIAGE_RETURN => {
            // Raw use of control characters in JSON string.
            let err =
              Error::new(ErrorType::E101, index_start, scanner.current().index + 1);
            json_document.errors.push(err);

            return Err(());
          }

          _ => {
            // Invalid escape character in JSON string.
            let err =
              Error::new(ErrorType::E116, index_start, scanner.current().index + 1);
            json_document.errors.push(err);

            return Err(());
          }
        }
      }

      HORIZONTAL_TAB | NEW_LINE | CARRIAGE_RETURN => {
        // Raw use of control characters in JSON string.
        let err = Error::new(ErrorType::E101, index_start, scanner.current().index + 1);
        json_document.errors.push(err);

        return Err(());
      }

      // Valid non escape or control character.
      _ => continue,
    }
  }
}

pub fn validate_string(
  json_document: &mut JSON,
  scanner: &mut Scanner,
) -> Result<(), ()> {
  match &json_document.last_parsed_token {
    Some(last_parsed_token) => match last_parsed_token {
      ParseTokens::BeginObject
      | ParseTokens::ValueSeparator
      | ParseTokens::NameSeparator
      | ParseTokens::BeginArray => match json_document.stack.last() {
        Some(token) => match token {
          StackTokens::NameSeparator => {
            json_document.stack.pop();
            json_document.object_has_valid_member = true;
            validate(json_document, scanner)
          }

          StackTokens::BeginObject => {
            json_document.object_has_valid_member = false;
            validate(json_document, scanner)
          }

          StackTokens::BeginArray => validate(json_document, scanner),
        },

        None => validate(json_document, scanner),
      },

      // Illegal string after structural token. Expected comma or colon.
      _ => {
        let last_parsed_index = scanner.current().index;
        let err = Error::new(ErrorType::E114, last_parsed_index, last_parsed_index + 1);
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
