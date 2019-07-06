mod helpers;

use jsonprima;

// Invalid JSON tokens.
test!(test_1, "q", vec![("E106", 0, 1)]);
