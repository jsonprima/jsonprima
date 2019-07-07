use crate::error::{Error, ErrorType};
use crate::tokens::{ParseTokens, Tokens};

// Helper macro to test if the next entry in tokens.iterator
// match expected_character.
macro_rules! next {
  ($tokens:ident, $expected_character:expr, $index_start:expr) => {
    let result = $tokens
      .iterator
      .next()
      .ok_or_else(|| {
        // No more graphemes to parse.
        let err = Error::new(
          ErrorType::E104,
          $index_start,
          $tokens.current_iterator_index + 1,
        );

        $tokens.errors.push(err);
        ()
      })
      .and_then(|(next_index, next_character)| {
        if next_character != $expected_character {
          // Invalid character in literal name.
          let err = Error::new(ErrorType::E105, $index_start, next_index + 1);
          $tokens.errors.push(err);
          $tokens.current_iterator_index = next_index;
          $tokens.current_iterator_character = next_character;

          Err(())
        } else {
          $tokens.current_iterator_index = next_index;
          $tokens.current_iterator_character = next_character;
          Ok(())
        }
      });

    if result.is_err() {
      return result;
    }
  };
}

fn validate(tokens: &mut Tokens) -> Result<(), ()> {
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
  let index_start = tokens.current_iterator_index;

  next!(tokens, 'a', index_start);
  next!(tokens, 'l', index_start);
  next!(tokens, 's', index_start);
  next!(tokens, 'e', index_start);

  // If none of the macro invocations returned error,
  // we have successfully validate the value.
  tokens.last_parsed_token = Some(ParseTokens::False);

  Ok(())
}

pub fn validate_false(tokens: &mut Tokens) -> Result<(), ()> {
  match &tokens.last_parsed_token {
    Some(last_parsed_token) => match last_parsed_token {
      ParseTokens::BeginArray => validate(tokens),

      ParseTokens::ValueSeparator => validate(tokens),

      // Illegal character "f" after structural token. Expected comma or colon.
      _ => {
        let last_parsed_index = tokens.current_iterator_index;
        let err = Error::new(ErrorType::E107, last_parsed_index, last_parsed_index + 1);
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
