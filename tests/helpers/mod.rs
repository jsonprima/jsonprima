use jsonprima;

// Helper macro to test against errors in validate function.
#[macro_export]
macro_rules! test {
  ($name:ident, $contents:expr, $expected_errors:expr) => {
    #[test]
    fn $name() {
      // Which errors do we expect?
      let mut expected_errors: Vec<(&str, usize, usize)> = $expected_errors;

      // Which is the JSON document to validate?
      let contents: &str = $contents;

      // Which are the actual errors returned from validate function?
      let mut actual_errors = jsonprima::validate(contents);

      // Sort errors by the start position.
      // We test expected_errors against actual_errors in parallel.
      expected_errors.sort_by(|a, b| a.1.cmp(&b.1));
      actual_errors.sort_by(|a, b| a.index_start.cmp(&b.index_start));

      // Check if there exist as many errors as we want to.
      // This check also fulfill the requirement that wants the
      // two vectors to have the same size to be checked in parallel.
      if actual_errors.len() != expected_errors.len() {
        panic!(
          "Expected to find {} errors by found {}. \n{:#?}",
          expected_errors.len(),
          actual_errors.len(),
          actual_errors
        );
      }

      // Turn each vector into an iterator.
      let mut iter_expected_errors = expected_errors.iter().peekable();
      let mut iter_actual_errors = actual_errors.into_iter();

      // For each error returned by the validate function...
      while let Some(actual_error) = iter_actual_errors.next() {
        // Get the parallel expected error.
        let (expected_error_code, expected_error_index_start, expected_error_index_end) =
          iter_expected_errors.next().unwrap();

        // Check if error codes match.
        if actual_error.err.code() != *expected_error_code {
          panic!(
            "Expected error code \"{}\" to equal \"{}\"",
            actual_error.err.code(),
            expected_error_code
          );
        }

        // Check if error start positions match.
        if actual_error.index_start != *expected_error_index_start {
          panic!(
            "Expected start error position {} to equal {}",
            actual_error.index_start, expected_error_index_start
          );
        }

        // Check if error end positions match.
        if actual_error.index_end != *expected_error_index_end {
          panic!(
            "Expected end error position {} to equal {}",
            actual_error.index_end, expected_error_index_end
          );
        }
      }
    }
  };
}
