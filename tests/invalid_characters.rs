mod helpers;

use jsonprima;

// Invalid JSON tokens.
test!(test_1, "q", vec![("E106", 0, 1)]);
test!(test_2, "  q", vec![("E106", 2, 3)]);
