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

// Nested arrays
test!(test_17, "[ [] ]", vec![]);
test!(test_18, " [ [ [ \n [ ]]]]", vec![]);

// Trailing end-array
test!(test_19, "[[[]]]] ", vec![("E126", 6, 7)]);
test!(test_20, " [[[ \n ]]]] ", vec![("E126", 10, 11)]);

// Unterminated nested arrays
test!(test_21, "[[[[[[[]]]] ", vec![("E127", 11, 12)]);
test!(test_22, " [[[\n\n[[[[[[ \n ]]]] ", vec![("E127", 19, 20)]);

// true as array element
test!(test_23, "[ true ]", vec![]);
test!(test_24, "[ [ true  ]]", vec![]);
test!(test_25, "[[[true]", vec![("E127", 7, 8)]);
test!(test_26, "[true]]]]", vec![("E126", 6, 7)]);

// false as array element
test!(test_27, "[ false ]", vec![]);
test!(test_28, "[ [ false  ]]", vec![]);
test!(test_29, "[[[false]", vec![("E127", 8, 9)]);
test!(test_30, "[false]]]]", vec![("E126", 7, 8)]);

// null as array element
test!(test_31, "[ null ]", vec![]);
test!(test_32, "[ [ null  ]]", vec![]);
test!(test_33, "[[[null]", vec![("E127", 7, 8)]);
test!(test_34, "[null]]]]", vec![("E126", 6, 7)]);
