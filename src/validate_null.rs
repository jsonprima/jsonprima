use crate::error::{Error, ErrorType};
use crate::tokens::{ParseTokens, Tokens};

// Helper macro to test if the next entry in tokens.iterator
// match expected_character.
macro_rules! next {
  ($tokens:ident, $expected_character:expr, $index_start:expr) => {
    let err = $tokens
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

    if err.is_err() {
      return err;
    }
  };
}

fn validate(tokens: &mut Tokens) -> Result<(), ()> {
  let index_start = tokens.current_iterator_index;

  next!(tokens, 'u', index_start);
  next!(tokens, 'l', index_start);
  next!(tokens, 'l', index_start);

  tokens.last_parsed_token = Some(ParseTokens::Null);

  Ok(())
}

pub fn validate_null(tokens: &mut Tokens) -> Result<(), ()> {
  match &tokens.last_parsed_token {
    Some(last_parsed_token) => match last_parsed_token {
      // Illegal character "n" after structural token. Expected comma or colon.
      _ => {
        let last_parsed_index = tokens.current_iterator_index;
        let err = Error::new(ErrorType::E108, last_parsed_index, last_parsed_index + 1);
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