pub struct CurrentEntry {
  pub index: usize,
  pub character: char,
}

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

  pub fn peek(&mut self) -> Option<&(usize, char)> {
    self.iterator.peek()
  }

  pub fn current(&self) -> &CurrentEntry {
    &self.current
  }
}
