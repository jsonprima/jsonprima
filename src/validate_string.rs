use crate::error::{Error, ErrorType};
use crate::tokens::{general_tokens::*, ParseTokens, StackTokens, Tokens};

// Given the tokens structure where the iterator index is in the
// start of a Unicode escape sequence values, i.e. X in \uXXXX,
// get the decimal representation of the escape sequence.
fn parse_escape_sequence(tokens: &mut Tokens, index_start: usize) -> Result<u32, Error> {
  // The decimal representation of the Unicode escape sequence.
  let mut uffff: u32 = 0;

  // Unicode escape sequence is defined
  // as \uxxxx, where x is ASCII Hex value.
  for _ in 0..4 {
    let next = tokens.iterator.next();

    if next.is_none() {
      return Err(Error::new(
        ErrorType::E104,
        index_start,
        tokens.current_iterator_index + 1,
      ));
    }

    tokens.current_iterator_index = next.unwrap().0;
    tokens.current_iterator_character = next.unwrap().1;

    if !tokens.current_iterator_character.is_ascii_hexdigit() {
      // Invalid character found in Unicode escape sequence.
      return Err(Error::new(
        ErrorType::E117,
        index_start,
        tokens.current_iterator_index + 1,
      ));
    }

    let hex = tokens.current_iterator_character.to_digit(16).unwrap();
    uffff = uffff * 16 + hex
  }

  // Return the decimal value of the Unicode escape sequence.
  Ok(uffff)
}

fn validate(tokens: &mut Tokens) -> Result<(), ()> {
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
  let index_start = tokens.current_iterator_index;

  // Parse all characters as string values until quotation mark.
  loop {
    let next = tokens.iterator.next();

    // End of string reached. We have successfully parse the JSON string.
    if next.filter(|(_, character)| *character == '"').is_some() {
      tokens.current_iterator_index = next.unwrap().0;
      tokens.current_iterator_character = '"';
      tokens.last_parsed_token = Some(ParseTokens::String);
      break Ok(());
    }

    // Invalid end of JSON document leaving the string unterminated.
    if next.is_none() {
      // No more characters to parse.
      let err = Error::new(
        ErrorType::E104,
        index_start,
        tokens.current_iterator_index + 1,
      );
      tokens.errors.push(err);

      break Err(());
    }

    // In case the character is not closing quotation mark and exists,
    // validate it based on the rules of RFC 8259.
    tokens.current_iterator_index = next.unwrap().0;
    tokens.current_iterator_character = next.unwrap().1;

    match tokens.current_iterator_character {
      '\\' => {
        // Start of escape character.
        // Read the next character to find out more.
        let next = tokens.iterator.next();

        if next.is_none() {
          // No more characters to parse.
          let err = Error::new(
            ErrorType::E104,
            index_start,
            tokens.current_iterator_index + 1,
          );
          tokens.errors.push(err);

          break Err(());
        }

        tokens.current_iterator_index = next.unwrap().0;
        tokens.current_iterator_character = next.unwrap().1;

        match tokens.current_iterator_character {
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
            let high_surrogate: u32;

            match parse_escape_sequence(tokens, index_start) {
              Ok(uffff) => {
                high_surrogate = uffff;
              }
              Err(err) => {
                tokens.errors.push(err);
                break Err(());
              }
            };

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
                let next = tokens.iterator.next();
                if next.is_none() {
                  // No more characters to parse.
                  let err = Error::new(
                    ErrorType::E119,
                    index_start,
                    tokens.current_iterator_index + 1,
                  );
                  tokens.errors.push(err);

                  break Err(());
                }

                tokens.current_iterator_index = next.unwrap().0;
                tokens.current_iterator_character = next.unwrap().1;

                if tokens.current_iterator_character != '\\' {
                  let err = Error::new(
                    ErrorType::E119,
                    index_start,
                    tokens.current_iterator_index + 1,
                  );
                  tokens.errors.push(err);

                  break Err(());
                }

                let next = tokens.iterator.next();
                if next.is_none() {
                  // No more characters to parse.
                  let err = Error::new(
                    ErrorType::E119,
                    index_start,
                    tokens.current_iterator_index + 1,
                  );
                  tokens.errors.push(err);

                  break Err(());
                }

                tokens.current_iterator_index = next.unwrap().0;
                tokens.current_iterator_character = next.unwrap().1;

                if tokens.current_iterator_character != 'u' {
                  let err = Error::new(
                    ErrorType::E119,
                    index_start,
                    tokens.current_iterator_index + 1,
                  );
                  tokens.errors.push(err);

                  break Err(());
                }

                let low_surrogate: u32;

                match parse_escape_sequence(tokens, index_start) {
                  Ok(uffff) => {
                    low_surrogate = uffff;
                  }
                  Err(err) => {
                    tokens.errors.push(err);
                    break Err(());
                  }
                };

                // Borrowed from https://stackoverflow.com/a/23920015
                let unicode_value = (high_surrogate << 10) + low_surrogate - 0x35f_dc00;

                if std::char::from_u32(unicode_value).is_none() {
                  // Invalid Unicode character in JSON string.
                  let err = Error::new(
                    ErrorType::E118,
                    index_start,
                    tokens.current_iterator_index + 1,
                  );
                  tokens.errors.push(err);

                  break Err(());
                }
              }
            }
          }

          HORIZONTAL_TAB | NEW_LINE | CARRIAGE_RETURN => {
            // Raw use of control characters in JSON string.
            let err = Error::new(
              ErrorType::E101,
              index_start,
              tokens.current_iterator_index + 1,
            );
            tokens.errors.push(err);

            break Err(());
          }

          _ => {
            // Invalid escape character in JSON string.
            let err = Error::new(
              ErrorType::E116,
              index_start,
              tokens.current_iterator_index + 1,
            );
            tokens.errors.push(err);

            break Err(());
          }
        }
      }

      HORIZONTAL_TAB | NEW_LINE | CARRIAGE_RETURN => {
        // Raw use of control characters in JSON string.
        let err = Error::new(
          ErrorType::E101,
          index_start,
          tokens.current_iterator_index + 1,
        );
        tokens.errors.push(err);

        break Err(());
      }

      // Valid non escape or control character.
      _ => continue,
    }
  }
}

pub fn validate_string(tokens: &mut Tokens) -> Result<(), ()> {
  match &tokens.last_parsed_token {
    Some(last_parsed_token) => match last_parsed_token {
      ParseTokens::BeginObject
      | ParseTokens::ValueSeparator
      | ParseTokens::NameSeparator
      | ParseTokens::BeginArray => match tokens.stack.last() {
        Some(token) => match token {
          StackTokens::NameSeparator => {
            tokens.stack.pop();
            tokens.object_has_valid_member = true;
            validate(tokens)
          }

          StackTokens::BeginObject => {
            tokens.object_has_valid_member = false;
            validate(tokens)
          }

          StackTokens::BeginArray => validate(tokens),
        },

        None => validate(tokens),
      },

      // Illegal string after structural token. Expected comma or colon.
      _ => {
        let last_parsed_index = tokens.current_iterator_index;
        let err = Error::new(ErrorType::E114, last_parsed_index, last_parsed_index + 1);
        tokens.errors.push(err);

        Err(())
      }
    },

    None => {
      tokens.root_value_parsed = true;
      validate(tokens)
    }
  }
}
