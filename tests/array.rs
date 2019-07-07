mod helpers;

use jsonprima;

// [
test!(test_1, "[", vec![("E127", 0, 1)]);
test!(test_2, " [", vec![("E127", 1, 2)]);
test!(test_3, "[ ", vec![("E127", 1, 2)]);
test!(test_4, " [ ", vec![("E127", 2, 3)]);

// ]
test!(test_5, "]", vec![("E126", 0, 1)]);
test!(test_6, " ]", vec![("E126", 1, 2)]);
test!(test_7, "] ", vec![("E126", 0, 1)]);
test!(test_8, " ] ", vec![("E126", 1, 2)]);

// []
test!(test_9, "[]", vec![]);
test!(test_10, " []", vec![]);
test!(test_11, "[] ", vec![]);
test!(test_12, " [ \n \n \r \t] ", vec![]);

// []]
test!(test_13, "[]]", vec![("E126", 2, 3)]);
test!(test_14, " []]", vec![("E126", 3, 4)]);
test!(test_15, "[]] ", vec![("E126", 2, 3)]);
test!(test_16, " []] ", vec![("E126", 3, 4)]);
