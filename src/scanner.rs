//! Scanner

/// Current index and character of the JSON document.
pub struct CurrentEntry {
  pub index: usize,
  pub character: char,
}

/// Create a scanner, a custom version of peekable and enumerate iterator
/// that can save info regarding to the current entry.
pub struct Scanner<'a> {
  iterator: std::iter::Peekable<std::iter::Enumerate<std::str::Chars<'a>>>,
  current: CurrentEntry,
}

impl<'a> Scanner<'a> {
  pub fn new(code: &'a str) -> Scanner<'a> {
    Scanner {
      iterator: code.chars().enumerate().peekable(),
      current: CurrentEntry {
        index: 0,
        character: '\0',
      },
    }
  }

  /// Calling `next()` advances the iterator to next entry and save
  /// it into `current` field.
  pub fn next(&mut self) -> Option<(usize, char)> {
    let next = self.iterator.next();
    match next {
      Some(ref item) => {
        self.current = CurrentEntry {
          index: item.0,
          character: item.1,
        };

        next
      }
      None => None,
    }
  }

  /// Get the next entry without advancing the scanner.
  pub fn peek(&mut self) -> Option<&(usize, char)> {
    self.iterator.peek()
  }

  /// Return the current entry.
  pub fn current(&self) -> &CurrentEntry {
    &self.current
  }
}
