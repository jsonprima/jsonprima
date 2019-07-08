use crate::error::{Error, ErrorType};
use crate::tokens::{ParseTokens, StackTokens, Tokens};

fn validate(tokens: &mut Tokens) -> Result<(), ()> {
  // A vector holding all the characters that will result in
  // JSON number.
  let mut num: Vec<char> = Vec::new();

  // Track next iterator entry.
  // Assume that the value of next() is None.
  let next: Option<(usize, char)> = None;

  // Save the position of the first character.
  // This will help us set a range that will highlight the whole incorrect value
  // in case of an error.
  //
  // Example:
  //
  // ```rust
  // // Invalid `-55e` root value in JSON document.
  // let text: &str = "-55e";
  // let errors = jsonprima::validate(&text);
  // println!("{:#?}", errors); // => [("E110", 0, 4)]
  // ```
  let index_start = tokens.current_iterator_index;

  // We already ensured there is at least a digit or minus sign.
  // In case there is only a minus sign, with no other digits following,
  // the next condition will successfully catch the error.
  num.push(tokens.current_iterator_character);

  // If the first character is zero, the next cannot be a number
  // (to avoid leading zeroes).
  if tokens.current_iterator_character == '0' {
    let peek = tokens.iterator.peek();

    if peek
      .filter(|(_, character)| character.is_ascii_digit())
      .is_some()
    {
      let last_parsed_index = peek.unwrap().0;
      let err = Error::new(ErrorType::E111, index_start, last_parsed_index + 1);
      tokens.errors.push(err);

      return Err(());
    }
  }

  // If the first character is minus sign,
  // ensure that the next character is ASCII digit,
  // but not leading zero.
  if tokens.current_iterator_character == '-' {
    let peek = tokens.iterator.peek();

    // Next entry does not exist.
    if peek.is_none() {
      let last_parsed_index = tokens.current_iterator_index;
      let err = Error::new(ErrorType::E110, index_start, last_parsed_index + 1);
      tokens.errors.push(err);

      return Err(());
    }

    // The next character after minus is non digit.
    if !peek.unwrap().1.is_ascii_digit() {
      let last_parsed_index = peek.unwrap().0;
      let err = Error::new(ErrorType::E110, index_start, last_parsed_index + 1);
      tokens.errors.push(err);

      return Err(());
    }

    // Next character after minus is zero.
    // This is okay only if it is not leading zero.
    if peek.unwrap().1 == '0' {
      // Advance the iterator to next entry and save the current character in num vector,
      // because is valid.
      let next = tokens.iterator.next();
      tokens.current_iterator_index = next.unwrap().0;
      tokens.current_iterator_character = next.unwrap().1;
      num.push(tokens.current_iterator_character);

      let peek = tokens.iterator.peek();
      if peek
        .filter(|(_, character)| character.is_ascii_digit())
        .is_some()
      {
        let last_parsed_index = peek.unwrap().0;
        let err = Error::new(ErrorType::E111, index_start, last_parsed_index + 1);
        tokens.errors.push(err);

        return Err(());
      }
    }
  }

  // Parse all digits before decimal point or exponential part.
  while tokens
    .iterator
    .peek()
    .filter(|(_, character)| character.is_ascii_digit())
    .is_some()
  {
    // Advance the iterator to next entry and save the current character in num vector,
    // because is valid.
    let next = tokens.iterator.next();
    tokens.current_iterator_index = next.unwrap().0;
    tokens.current_iterator_character = next.unwrap().1;
    num.push(tokens.current_iterator_character);
  }

  // Parse decimal point.
  if tokens
    .iterator
    .peek()
    .filter(|(_, character)| character == &'.')
    .is_some()
  {
    // Advance the iterator to next entry and save the current character in num vector,
    // because is valid.
    let next = tokens.iterator.next();
    tokens.current_iterator_index = next.unwrap().0;
    tokens.current_iterator_character = next.unwrap().1;
    num.push(tokens.current_iterator_character);

    // First character after decimal point, must be ASCII digit.
    let peek = tokens.iterator.peek();

    // Next entry does not exist.
    if peek.is_none() {
      let last_parsed_index = tokens.current_iterator_index;
      let err = Error::new(ErrorType::E110, index_start, last_parsed_index + 1);
      tokens.errors.push(err);

      return Err(());
    }

    // The next character after minus is non digit.
    if !peek.unwrap().1.is_ascii_digit() {
      let last_parsed_index = peek.unwrap().0;
      let err = Error::new(ErrorType::E110, index_start, last_parsed_index + 1);
      tokens.errors.push(err);

      return Err(());
    }

    // Parse all digits after decimal point.
    while tokens
      .iterator
      .peek()
      .filter(|(_, character)| character.is_ascii_digit())
      .is_some()
    {
      // Advance the iterator to next entry and save the current character in num vector,
      // because is valid.
      let next = tokens.iterator.next();
      tokens.current_iterator_index = next.unwrap().0;
      tokens.current_iterator_character = next.unwrap().1;
      num.push(tokens.current_iterator_character);
    }
  }

  // Parse exponential part, if exist.
  if tokens
    .iterator
    .peek()
    .filter(|(_, character)| character == &'e' || character == &'E')
    .is_some()
  {
    // Advance the iterator to next entry and save the current character in num vector,
    // because is valid.
    let next = tokens.iterator.next();
    tokens.current_iterator_index = next.unwrap().0;
    tokens.current_iterator_character = next.unwrap().1;
    num.push(tokens.current_iterator_character);

    // Exponent part can be accompanied by a a plus or minus sign.
    if tokens
      .iterator
      .peek()
      .filter(|(_, character)| character == &'+' || character == &'-')
      .is_some()
    {
      // Advance the iterator to next entry and save the current character in num vector,
      // because is valid.
      let next = tokens.iterator.next();
      tokens.current_iterator_index = next.unwrap().0;
      tokens.current_iterator_character = next.unwrap().1;
      num.push(tokens.current_iterator_character);
    }

    // First character of exponential part, must be ASCII digit.
    let peek = tokens.iterator.peek();

    // Next entry does not exist.
    if peek.is_none() {
      let last_parsed_index = tokens.current_iterator_index;
      let err = Error::new(ErrorType::E110, index_start, last_parsed_index + 1);
      tokens.errors.push(err);

      return Err(());
    }

    // The next character after minus is non digit.
    if !peek.unwrap().1.is_ascii_digit() {
      let last_parsed_index = peek.unwrap().0;
      let err = Error::new(ErrorType::E110, index_start, last_parsed_index + 1);
      tokens.errors.push(err);

      return Err(());
    }

    // Parse all digits of exponential part.
    while tokens
      .iterator
      .peek()
      .filter(|(_, character)| character.is_ascii_digit())
      .is_some()
    {
      // Advance the iterator to next entry and save the current character in num vector,
      // because is valid.
      let next = tokens.iterator.next();
      tokens.current_iterator_index = next.unwrap().0;
      tokens.current_iterator_character = next.unwrap().1;
      num.push(tokens.current_iterator_character);
    }
  }

  // Parse the JSON number from the `num` vector.
  match num.iter().collect::<String>().parse::<f64>() {
    Ok(num) => {
      // JSON numbers must be in range [-(2**53)+1, (2**53)-1]
      // to be interoperable.
      if num < core::f64::MIN || num > core::f64::MAX {
        // Could not parse out of range JSON number.
        if next.is_none() {
          let last_parsed_index = tokens.current_iterator_index;
          let err = Error::new(ErrorType::E112, index_start, last_parsed_index + 1);
          tokens.errors.push(err);
        } else {
          let last_parsed_index = next.unwrap().0;
          let err = Error::new(ErrorType::E112, index_start, last_parsed_index + 1);
          tokens.errors.push(err);
        }

        Err(())
      } else {
        tokens.last_parsed_token = Some(ParseTokens::Number);
        Ok(())
      }
    }
    Err(_) => {
      // Could not parse JSON number.
      if next.is_none() {
        let last_parsed_index = tokens.current_iterator_index;
        let err = Error::new(ErrorType::E113, index_start, last_parsed_index + 1);
        tokens.errors.push(err);
      } else {
        let last_parsed_index = next.unwrap().0;
        let err = Error::new(ErrorType::E113, index_start, last_parsed_index + 1);
        tokens.errors.push(err);
      }

      Err(())
    }
  }
}

pub fn validate_number(tokens: &mut Tokens) -> Result<(), ()> {
  match &tokens.last_parsed_token {
    Some(last_parsed_token) => match last_parsed_token {
      ParseTokens::BeginArray => validate(tokens),

      ParseTokens::ValueSeparator | ParseTokens::BeginObject => {
        if tokens.stack.last().unwrap_or(&StackTokens::BeginArray)
          == &StackTokens::BeginObject
        {
          let last_parsed_index = tokens.current_iterator_index;
          let err = Error::new(ErrorType::E137, last_parsed_index, last_parsed_index + 1);
          tokens.errors.push(err);

          Err(())
        } else {
          validate(tokens)
        }
      }

      ParseTokens::NameSeparator => {
        tokens.stack.pop();
        tokens.object_has_valid_member = true;
        validate(tokens)
      }

      // Illegal number after structural token. Expected comma or colon.
      _ => {
        let last_parsed_index = tokens.current_iterator_index;
        let err = Error::new(ErrorType::E109, last_parsed_index, last_parsed_index + 1);
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
