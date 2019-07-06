use crate::error::Error;

// List of structural tokens defined in RFC 8259
// and pseudo tokens, such as whitespace characters.
pub mod general_tokens {
  // begin-array = ws %x5B ws ; `[` left square bracket
  pub const BEGIN_ARRAY: char = '[';

  // begin-object = ws %x7B ws ; `{` left curly bracket
  pub const BEGIN_OBJECT: char = '{';

  // end-array = ws %x5D ws ; `]` right square bracket
  pub const END_ARRAY: char = ']';

  // end-object = ws %x7D ws ; `}` right curly bracket
  pub const END_OBJECT: char = '}';

  // quotation-mark = ws %x22 ws ; `"` quotation-mark
  pub const QUOTATION_MARK: char = '"';

  // name-separator = ws %x3A ws ; `:` colon
  pub const NAME_SEPARATOR: char = ':';

  // value-separator = ws %x2C ws ; `,` comma
  pub const VALUE_SEPARATOR: char = ',';

  // Space = ws %x20 ws ; ` ` space
  pub const SPACE: char = ' ';

  // Horizontal Tab = ws %x09 ws ;
  pub const HORIZONTAL_TAB: char = '\t';

  // Line feed or New line = ws %x0A ws ;
  pub const NEW_LINE: char = '\n';

  // Carriage return = ws %x0D ws ;
  pub const CARRIAGE_RETURN: char = '\r';
}

// Enumerate structural tokens and JSON values
// to track each corresponding parsed JSON structural
// token and value.
pub enum ParseTokens {
  BeginArray,
  BeginObject,
  EndArray,
  EndObject,
  String,
  NameSeparator,
  ValueSeparator,
  True,
  False,
  Null,
  Number,
}

// Validate nested JSON values using a stack that track the
// current level of nesting.
//
// Suppose that we have to parse the following JSON value:
//   [[ true ]]
// To track the nesting of `true` we save begin-array
// tokens into a stack like so:
//   Stack = [StackTokens::BeginArray, StackTokens::BeginArray]
// This help us identify if each nested array level contains a
// valid set of begin-array/end-array tokens.
//
// StackTokens::NameSeparator help us identify valid object members.
// For instance suppose that we have to parse the following JSON value:
//   { "hello": "invalid": "world "}
// After parsing successfully the member "hello": "invalid" the stack is:
//   Stack = [StackTokens::BeginObject, StackTokens::NameSeparator]
// In the next iteration we check the name-separator after "invalid"
// with the last parsed token in the stack. We see that we have already
// parsed an object member and this token is not valid.
pub enum StackTokens {
  BeginArray,
  BeginObject,
  NameSeparator,
}

// Helper structure to track all the info used by the validator,
// function such as last parsed token, the current iteration state, etc.
// This structure is big and do multiple things. We could separate
// it into multiple structures and be SoC, but keeping all the info
// into a single one, we manage easily to pass it around functions,
// making the compiler happy.
// This is not idiomatic Rust and we might reconsider our approach in
// future releases, even more if the size of this library starts to
// increase.
pub struct Tokens<'a> {
  // The last parsed JSON token.
  // In case the JSON document is empty, the value is None.
  pub last_parsed_token: Option<ParseTokens>,

  // Create a character iterator from the code input provided
  // as parameter to validate() function call.
  // Make the iterator enumerable to track each character index
  // and peekable to allow test for the next iterator entry.
  //
  // Note on the use of peekable:
  // While we do not plan to use peek() very much, calling the
  // method before advancing using next() has a performance impact
  // as we have to double check the iterator entry.
  // Calling peek() every time before calling next() doubles the
  // execution time. This is not the case in our application, but we
  // still expect to get a minor performance drain.
  // Until we find a better way of parsing without the need of peek(),
  // we'll keep it that way.
  pub iterator: std::iter::Peekable<std::iter::Enumerate<std::str::Chars<'a>>>,

  // The current iterator index and character after the last
  // call to iterator.next()
  pub current_iterator_index: usize,
  pub current_iterator_character: char,

  // Check if the root JSON value has been parsed.
  // Help us catch trailing commas on root values.
  pub root_value_parsed: bool,

  // Stack of current nesting level tokens.
  pub stack: Vec<StackTokens>,

  // Check if a valid member exist in an object.
  pub object_has_valid_member: bool,

  // Track the validation errors.
  // This structure is returned by validate() function to
  // public API.
  // Error is defined in crate::error::Error
  pub errors: Vec<Error>,
}

impl<'a> Tokens<'a> {
  // Returns a new Token instance.
  pub fn new(
    iterator: std::iter::Peekable<std::iter::Enumerate<std::str::Chars<'a>>>,
  ) -> Tokens<'a> {
    Tokens {
      last_parsed_token: None,
      root_value_parsed: false,
      stack: Vec::new(),
      object_has_valid_member: false,
      iterator,
      current_iterator_index: 0,
      current_iterator_character: '\0',
      errors: Vec::new(),
    }
  }
}
